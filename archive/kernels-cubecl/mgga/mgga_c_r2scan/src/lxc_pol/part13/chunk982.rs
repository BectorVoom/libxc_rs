//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 982/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk982<F: Float>(t11523: F, t3271: F, t10619: F, t3579: F, t10615: F, t3275: F, t3582: F, t2847: F, t797: F, t3276: F, t3696: F, t860: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11524 = t11523 * t3271;
    let t11525 = t11524 / F::cast_from(4.0_f64);
    let t11526 = t3579 * t10619;
    let t11527 = t11526 / F::cast_from(4.0_f64);
    let t11529 = t3275 * t10615 * t3582;
    let t11530 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t11529;
    let t11531 = t797 * t2847;
    let t11533 = t3275 * t3276 * t11531;
    let t11534 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t11533;
    let t11535 = t860 * t3696;
    (t11524, t11525, t11526, t11527, t11529, t11530, t11531, t11533, t11534, t11535)
}
