//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 268/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk268<F: Float>(t899: F, t902: F, t120: F, t307: F, t328: F, t332: F, t319: F, t97: F, t99: F, t315: F, t324: F, t122: F, t331: F, t330: F, t101: F, t296: F, t299: F, t304: F, t308: F, t316: F, t333: F, t647: F, t654: F, t661: F, t665: F, t870: F, t871: F, t875: F, t880: F, t885: F, t890: F, t891: F, t895: F, t896: F, tau0: F) -> (F, F, F, F, F, F, F) {
    let t903 = t899 * t902;
    let t907 = t328 * t307 * t120;
    let t908 = t907 * t332;
    let t909 = t319 * t97;
    let t911 = 1.0 / t99 / t909;
    let t912 = t315 * t911;
    let t913 = t324 * tau0;
    let t914 = t912 * t913;
    let t918 = 1.0 / t331 / t122;
    let t919 = t330 * t918;
    let t930 = -0.125104062565404384e1 * t296 * t647 * t299 + 0.58691349263882304531e0 * t870 * t654 * t871 + 5.0 / 3.0 * t875 * t661 + 5.0 / 3.0 * t304 * t665 + 10.0 / 3.0 * t880 * t665 + 10.0 / 3.0 * t308 * t885 * t101 - 0.17058312527037532642e0 * t316 * t891 + 0.80027407411602181738e-1 * t896 * t903 + 0.7107630219598971934e-1 * t908 * t914 + 0.7107630219598971934e-1 * t919 * t914 - 0.17058312527037532642e0 * t333 * t315 * t890 * t324 + 0.80027407411602181738e-1 * t333 * t895 * t899 * t902;
    (t907, t908, t909, t913, t918, t919, t930)
}
