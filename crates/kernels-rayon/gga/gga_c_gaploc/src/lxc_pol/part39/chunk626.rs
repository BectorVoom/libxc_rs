//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 626/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk626(t10268: f64, t4820: f64, t6824: f64, t2478: f64, t993: f64, t6576: f64, t2890: f64, t6583: f64, t1339: f64, t3338: f64, t590: f64, t2482: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10396 = t4820 * t10268;
    let t10398 = 0.79445533226334281487e-1_f64 * t6824 * t10396;
    let t10399 = t993 * t2478;
    let t10400 = t6576 * t10399;
    let t10401 = 0.19171462976960374838e0_f64 * t10400;
    let t10402 = t2890 * t2478;
    let t10403 = t6583 * t10402;
    let t10404 = 0.19171462976960374838e0_f64 * t10403;
    let t10405 = t1339 * t3338;
    let t10406 = t10405 * t590;
    let t10409 = t993 * t2482;
    (t10398, t10401, t10404, t10405, t10406, t10409)
}
