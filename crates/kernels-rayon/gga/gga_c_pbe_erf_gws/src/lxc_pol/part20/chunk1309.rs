//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1309/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1309(t15342: f64, t53774: f64, t3888: f64, t859: f64, t13792: f64, t52990: f64, t13808: f64, t15186: f64, t13917: f64, t343: f64, t53799: f64, t54590: f64, t824: f64) -> (f64, f64, f64, f64, f64) {
    let t56740 = t53774 * t15342;
    let t56742 = t859 * t3888;
    let t56743 = t13792 * t56742;
    let t56745 = t52990 * t15342;
    let t56747 = t13808 * t15186;
    let t56753 = t13917 * t53799 * t824 * t54590 * t343;
    (t56740, t56743, t56745, t56747, t56753)
}
