//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 866/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk866(t2280: f64, t364: f64, t6275: f64, t2288: f64, t881: f64, t2259: f64, t2279: f64, t2298: f64, t2318: f64, t6112: f64, t6136: f64, t6139: f64, t6146: f64, t6196: f64, t6204: f64, t6207: f64, t6263: f64, t6266: f64, t6269: f64, t6272: f64, t6276: f64, t6279: f64, t6282: f64, t6283: f64, t6288: f64, t863: f64, t882: f64, t891: f64) -> (f64, f64, f64, f64) {
    let t6290 = 1.0_f64 / t2280 / t364;
    let t6291 = t6275 * t6290;
    let t6294 = t2288 * t881;
    let t6297 = t6112 + 1.0_f64 * t863 * t6263 - 0.35089341735807877242e1_f64 * t6266 * t2298 + 0.35089341735807877242e1_f64 * t2318 * t6269 - 6.0_f64 * t6272 * t2259 + 6.0_f64 * t2279 * t6276 - t6136 - t6139 + t6146 - t6196 - t6204 - t6207 + 0.5848223622634646207e0_f64 * t882 * t6279 + 0.10254018858216406658e4_f64 * t6282 * t6283 + 0.2069040516770936012e4_f64 * t6288 * t6291 + 0.17544670867903938621e1_f64 * t6294 * t891;
    (t6290, t6291, t6294, t6297)
}
