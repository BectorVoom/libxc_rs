//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1261/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1261(t1802: f64, t3089: f64, t3717: f64, t1285: f64, t5326: f64, t7623: f64, t17523: f64, t26842: f64, t3594: f64, t7616: f64, t3670: f64, t8184: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104706 = sigma2 * t1802;
    let t104707 = t104706 * t3089;
    let t104708 = t3717 * t104707;
    let t104721 = t1285 * t104707;
    let t104752 = t5326 * t7623;
    let t104758 = t3594 * t26842 * t17523;
    let t104762 = t3594 * t7616 * t17523;
    let t104818 = t3670 * t8184;
    (t104708, t104721, t104752, t104758, t104762, t104818)
}
