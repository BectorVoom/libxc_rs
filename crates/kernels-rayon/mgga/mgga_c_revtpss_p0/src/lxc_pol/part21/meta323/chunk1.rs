//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1601/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1601(t11002: f64, t11053: f64, t2408: f64, t890: f64, t2410: f64, t261: f64, t2411: f64, t2832: f64, t892: f64, t10552: f64, t10554: f64, t10557: f64, t10560: f64, t10562: f64, t10564: f64, t10627: f64, t1940: f64, t198: f64, t207: f64, t2394: f64, t2403: f64, t2404: f64, t2430: f64, t262: f64, t4541: f64, t775: f64, t9394: f64) -> (f64, f64, f64, f64, f64) {
    let t11054 = t11002 + t11053;
    let t11061 = t2408 * t890;
    let t11064 = 1.0_f64 / t2410 / t261;
    let t11071 = t890 * t2411;
    let t11075 = t2832 * t892;
    let t11082 = t11054 * t198 * t207 * t892 + 2.0_f64 * t11061 * t11064 * t198 * t207 + 6.0_f64 * t10627 * t198 * t262 - 3.0_f64 * t11071 * t1940 * t2832 + 9.0_f64 * t11075 * t2403 * t775 + 18.0_f64 * t2394 * t2404 * t4541 + 9.0_f64 * t2403 * t2404 * t2430 - t10552 + t10554 + t10557 + t10560 + t10562 + t10564 + t9394;
    (t11054, t11061, t11064, t11075, t11082)
}
