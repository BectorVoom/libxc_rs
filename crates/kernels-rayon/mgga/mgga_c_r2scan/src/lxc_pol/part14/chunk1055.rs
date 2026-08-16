//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1055/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1055(t10659: f64, t10943: f64, t3428: f64, t3430: f64, t6818: f64, t260: f64, t6100: f64, t1102: f64, t1104: f64, t3314: f64, t875: f64, t10648: f64, t10651: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37444 = t10943 * t10659;
    let t37447 = t6818 * t3428 * t3430;
    let t37449 = t260 * t6100;
    let t37451 = t1102 * t37449 * t1104;
    let t37453 = t3314 * t875;
    let t37455 = t10648 * t37453 * t10651;
    (t37444, t37447, t37449, t37451, t37453, t37455)
}
