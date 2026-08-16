//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 470/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk470(t2846: f64, t273: f64, t240: f64, t68: f64, t281: f64, t283: f64, t698: f64, t931: f64, t1014: f64, t913: f64, t275: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2892 = 0.39862222222222222223e0_f64 * t2846;
    let t2897 = 1.0_f64/f64::sqrt(t273);
    let t2902 = t68 * t240;
    let t2904 = t281 * t2902 * t283;
    let t2905 = 0.13692777777777777778e0_f64 * t2904;
    let t2906 = t698 * t931;
    let t2908 = t240 * t1014;
    let t2922 = t913 * t913;
    let t2923 = 1.0_f64 / t2922;
    let t2924 = t275 * t2923;
    let t2925 = t290 * t290;
    (t2892, t2897, t2902, t2904, t2905, t2906, t2908, t2924, t2925)
}
