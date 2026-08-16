//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2163/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2163(t1484: f64, t2249: f64, t4119: f64, t606: f64, t1408: f64, t2749: f64, t10143: f64, t7540: f64, t13191: f64, t25014: f64, t13196: f64, t13471: f64, t25: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87953 = t2249 * t1484;
    let t87957 = t606 * t4119;
    let t87961 = t1408 * t2749;
    let t87975 = t7540 * t10143;
    let t87978 = t25014 * t13191;
    let t87981 = t25014 * t13196;
    let t87984 = t25 * t13471;
    (t87953, t87957, t87961, t87975, t87978, t87981, t87984)
}
