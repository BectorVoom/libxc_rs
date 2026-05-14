//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 830/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk830<F: Float>(t2280: F, t364: F, t6275: F, t2288: F, t881: F, t2259: F, t2279: F, t2298: F, t2318: F, t6112: F, t6136: F, t6139: F, t6146: F, t6196: F, t6204: F, t6207: F, t6263: F, t6266: F, t6269: F, t6272: F, t6276: F, t6279: F, t6282: F, t6283: F, t6288: F, t863: F, t882: F, t891: F) -> (F, F, F, F) {
    let t6290 = 1.0 / t2280 / t364;
    let t6291 = t6275 * t6290;
    let t6294 = t2288 * t881;
    let t6297 = t6112 + 1.0 * t863 * t6263 - 0.35089341735807877242e1 * t6266 * t2298 + 0.35089341735807877242e1 * t2318 * t6269 - 6.0 * t6272 * t2259 + 6.0 * t2279 * t6276 - t6136 - t6139 + t6146 - t6196 - t6204 - t6207 + 0.5848223622634646207e0 * t882 * t6279 + 0.10254018858216406658e4 * t6282 * t6283 + 0.2069040516770936012e4 * t6288 * t6291 + 0.17544670867903938621e1 * t6294 * t891;
    (t6290, t6291, t6294, t6297)
}
