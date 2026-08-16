//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 943/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk943(t2478: f64, t993: f64, t6576: f64, t2890: f64, t6583: f64, t2482: f64, t9263: f64, t9422: f64, t9442: f64, t9446: f64, t9451: f64, t1415: f64, t2897: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10399 = t993 * t2478;
    let t10400 = t6576 * t10399;
    let t10401 = 0.19171462976960374838e0_f64 * t10400;
    let t10402 = t2890 * t2478;
    let t10403 = t6583 * t10402;
    let t10404 = 0.19171462976960374838e0_f64 * t10403;
    let t10409 = t993 * t2482;
    let t10410 = t9263 * t10409;
    let t10411 = 0.38342925953920749676e0_f64 * t10410;
    let t10412 = 0.63904876589867916128e-1_f64 * t9422;
    let t10414 = 0.15976219147466979032e-1_f64 * t9442;
    let t10415 = 0.31952438294933958064e-1_f64 * t9446;
    let t10416 = 0.31952438294933958064e-1_f64 * t9451;
    let t10421 = t1415 * t2897;
    (t10399, t10401, t10402, t10404, t10409, t10411, t10412, t10414, t10415, t10416, t10421)
}
