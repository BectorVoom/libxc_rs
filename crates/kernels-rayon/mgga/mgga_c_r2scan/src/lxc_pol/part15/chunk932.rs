//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 932/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk932(t10722: f64, t1592: f64, t3308: f64, t6541: f64, t574: f64, t10707: f64, t546: f64) -> (f64, f64, f64, f64) {
    let t10723 = t1592 * t10722;
    let t10725 = t3308 * t6541;
    let t10726 = t574 * t10725;
    let t10728 = t546 * t10707;
    (t10723, t10725, t10726, t10728)
}
