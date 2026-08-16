//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1890/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1890(t25956: f64, t26087: f64, t532: f64, t1450: f64, t2014: f64, t118: f64, t2011: f64, t2322: f64, t2331: f64, t2372: f64, t25800: f64, t25804: f64, t25805: f64, t25835: f64, t25838: f64, t25840: f64, t25842: f64, t25844: f64, t25846: f64, t25853: f64, t25858: f64, t25860: f64, t25863: f64, t25868: f64, t25872: f64, t4151: f64, t569: f64, t651: f64, t671: f64, t6985: f64, t7007: f64) -> (f64, f64, f64, f64) {
    let t26088 = t25956 + t26087;
    let t26089 = t532 * t26088;
    let t26090 = t26089 * t1450;
    let t26091 = t2014 * t26090;
    let t26092 = -t118 * t25800 + t2011 * t4151 - 4.0_f64 * t2322 * t7007 - 4.0_f64 * t2331 * t6985 - 2.0_f64 * t2372 * t6985 - 4.0_f64 * t25805 * t671 + t25835 * t569 - 4.0_f64 * t25872 * t651 - t25804 + t25838 - t25840 - t25842 - t25844 + t25846 - t25853 - t25858 - t25860 - t25863 + t25868 + t26091;
    (t26088, t26089, t26090, t26092)
}
