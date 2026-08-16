//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1242/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1242<F: Float>(t4025: F, t8319: F, t510: F, t19456: F, t8326: F, t26114: F, t26117: F, t31717: F, t7467: F, t26135: F, t8601: F, t12725: F) -> (F, F, F, F, F, F, F, F) {
    let t120112 = t4025 * t8319;
    let t120114 = F::cast_from(2.0_f64) * t120112 * t510;
    let t120120 = t19456 * t8326;
    let t120121 = F::cast_from(2.0_f64) * t120120;
    let t120122 = t26114 * t8326;
    let t120123 = F::cast_from(2.0_f64) * t120122;
    let t120124 = t26117 * t8326;
    let t120125 = F::cast_from(2.0_f64) * t120124;
    let t120127 = F::cast_from(4.0_f64) * t31717 * t7467;
    let t120129 = F::cast_from(4.0_f64) * t8601 * t26135;
    let t120130 = t12725 * t8326;
    (t120112, t120114, t120121, t120123, t120125, t120127, t120129, t120130)
}
