//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 363/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk363<F: Float>(t432: F, t427: F, t1086: F, t1111: F, t1092: F, t1103: F, t1108: F, t1115: F) -> (F, F, F, F) {
    let t1127 = t432 * t432;
    let t1128 = F::cast_from(1.0_f64) / t1127;
    let t1129 = t427 * t1128;
    let t1131 = F::cast_from(0.516475e0_f64) * t1086;
    let t1134 = F::cast_from(0.104195e0_f64) * t1111;
    let t1136 = F::cast_from(0.3529725e1_f64) * t1103 - t1131 + F::cast_from(0.516475e0_f64) * t1092 + F::cast_from(0.6311625e0_f64) * t1108 - t1134 + F::cast_from(0.104195e0_f64) * t1115;
    (t1127, t1128, t1129, t1136)
}
