//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1296/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1296(t14733: f64, t8700: f64, t14113: f64, t15204: f64, t15342: f64, t53774: f64, t3888: f64, t859: f64, t13792: f64, t52990: f64, t13808: f64, t15186: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56724 = t14733 * t8700;
    let t56728 = t14113 * t15204;
    let t56740 = t53774 * t15342;
    let t56742 = t859 * t3888;
    let t56743 = t13792 * t56742;
    let t56745 = t52990 * t15342;
    let t56747 = t13808 * t15186;
    (t56724, t56728, t56740, t56743, t56745, t56747)
}
