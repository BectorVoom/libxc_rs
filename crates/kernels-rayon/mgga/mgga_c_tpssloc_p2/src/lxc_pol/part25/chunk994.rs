//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 994/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk994(t6943: f64, t835: f64, t1336: f64, t1354: f64, t3858: f64, t6945: f64, t1339: f64, t3851: f64, t6936: f64, t3856: f64, t3788: f64, t3793: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22764 = t6943 * t835;
    let t22765 = t1336 * t22764;
    let t22766 = t22765 * t1354;
    let t22768 = t6945 * t3858;
    let t22770 = t1339 * t3851;
    let t22771 = t6936 * t22770;
    let t22773 = t1339 * t3856;
    let t22774 = t6936 * t22773;
    let t22776 = t3788 * t3793;
    (t22764, t22765, t22766, t22768, t22770, t22771, t22773, t22774, t22776)
}
