//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 717/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk717(t11306: f64, t419: f64, t11050: f64, t1527: f64, t11287: f64, t11294: f64, t11297: f64, t11299: f64, t11301: f64, t11304: f64, t8099: f64, t8110: f64, t8113: f64, t8116: f64, t8133: f64) -> (f64, f64, f64) {
    let t11307 = t419 * t11306;
    let t11309 = t1527 * t11050;
    let t11310 = t419 * t11309;
    let t11312 = -0.45399899292181069959e-1_f64 * t11287 - 0.42562405586419753086e-2_f64 * t8099 - 0.28374937057613168724e-2_f64 * t8110 + 0.21281202793209876543e-2_f64 * t8113 + 0.28374937057613168724e-2_f64 * t8116 - 0.1134997482304526749e-1_f64 * t8133 + 0.62424861526748971195e-1_f64 * t11294 - t11297 - 0.14187468528806584362e-2_f64 * t11299 - 0.68099848938271604939e-1_f64 * t11301 - 0.2979368391049382716e-1_f64 * t11304 - 0.51074886703703703704e-1_f64 * t11307 + 0.38306165027777777778e-1_f64 * t11310;
    (t11307, t11310, t11312)
}
