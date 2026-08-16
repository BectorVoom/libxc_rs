//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 234/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk234(t316: f64, t880: f64, t243: f64, t75: f64, t288: f64, t98: f64, t100: f64, t229: f64, t277: f64, t224: f64, t244: f64, t272: f64, t687: f64, t791: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t882 = 0.65854491829355115987e0_f64 * t316 * t880;
    let t883 = t243 * t75;
    let t884 = t883 * t288;
    let t886 = 1.0_f64 / t98;
    let t893 = 1.0_f64 / t100;
    let t904 = t229 * t277;
    let t905 = 8.0_f64 * t904;
    let t906 = t224 * t244;
    let t912 = t791 * t687 * t272;
    (t882, t883, t884, t886, t893, t905, t906, t912)
}
