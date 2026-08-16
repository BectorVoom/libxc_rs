//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 356/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk356<F: Float>(t1149: F, t1150: F, t1131: F, t1118: F, t1124: F, t431: F) -> (F, F, F, F, F, F) {
    let t1151 = t1149 * t1150;
    let t1153 = F::cast_from(1.0_f64) * t1131 * t1151;
    let t1154 = F::cast_from(0.17123333333333333333e-1_f64) * t1118;
    let t1156 = -t1154 + F::cast_from(0.17123333333333333333e-1_f64) * t1124;
    let t1159 = t431 * t431;
    let t1160 = F::cast_from(1.0_f64) / t1159;
    (t1151, t1153, t1154, t1156, t1159, t1160)
}
