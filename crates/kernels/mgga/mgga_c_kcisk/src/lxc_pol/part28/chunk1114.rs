//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1114/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1114<F: Float>(t2677: F, t31932: F, t9311: F, t9320: F, t9307: F, t9315: F, t3058: F, t3934: F, t9318: F, t31899: F, t31903: F, t31906: F, t31911: F, t31913: F, t31918: F, t31922: F, t31925: F, t31929: F) -> (F, F) {
    let t31933 = t2677 * t31932;
    let t31935 = t9311 * t9320;
    let t31937 = t9311 * t9307;
    let t31939 = t9315 * t9320;
    let t31941 = t9315 * t9307;
    let t31944 = t3934 * t9318 * t3058;
    let t31945 = t2677 * t31944;
    let t31947 = -0.8041666666666666667e-2 * t31899 - 0.20833333333333333334e-1 * t31903 - 0.18763888888888888889e-1 * t31906 - 0.120625e-1 * t31911 - 0.20833333333333333334e-1 * t31913 - 0.20833333333333333334e-1 * t31918 - 0.8101851851851851852e-1 * t31922 + 0.48611111111111111112e-1 * t31925 + 0.48611111111111111112e-1 * t31929 + 0.10416666666666666667e-1 * t31933 + 0.20833333333333333334e-1 * t31935 + 0.20833333333333333334e-1 * t31937 - 0.48611111111111111112e-1 * t31939 - 0.48611111111111111112e-1 * t31941 + 0.10416666666666666667e-1 * t31945;
    (t31944, t31947)
}
