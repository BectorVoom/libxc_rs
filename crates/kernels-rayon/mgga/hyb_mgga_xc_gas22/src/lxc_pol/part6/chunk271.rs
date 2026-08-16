//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 271/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk271(t330: f64, t918: f64, t101: f64, t296: f64, t299: f64, t304: f64, t308: f64, t315: f64, t316: f64, t324: f64, t333: f64, t647: f64, t654: f64, t661: f64, t665: f64, t870: f64, t871: f64, t875: f64, t880: f64, t885: f64, t890: f64, t891: f64, t895: f64, t896: f64, t899: f64, t902: f64, t903: f64, t908: f64, t914: f64) -> (f64, f64) {
    let t919 = t330 * t918;
    let t930 = -0.125104062565404384e1_f64 * t296 * t647 * t299 + 0.58691349263882304531e0_f64 * t870 * t654 * t871 + 5.0_f64 / 3.0_f64 * t875 * t661 + 5.0_f64 / 3.0_f64 * t304 * t665 + 10.0_f64 / 3.0_f64 * t880 * t665 + 10.0_f64 / 3.0_f64 * t308 * t885 * t101 - 0.17058312527037532642e0_f64 * t316 * t891 + 0.80027407411602181738e-1_f64 * t896 * t903 + 0.7107630219598971934e-1_f64 * t908 * t914 + 0.7107630219598971934e-1_f64 * t919 * t914 - 0.17058312527037532642e0_f64 * t333 * t315 * t890 * t324 + 0.80027407411602181738e-1_f64 * t333 * t895 * t899 * t902;
    (t919, t930)
}
