//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1135/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1135(t27157: f64, t27158: f64, t32946: f64, t574: f64, t3450: f64, t5899: f64, t9432: f64, t2185: f64, t23657: f64, t27152: f64, t28: f64, t32709: f64, t3408: f64, t89: f64) -> (f64, f64, f64, f64) {
    let t148381 = t27157 * t574 * t32946 * t27158;
    let t148385 = t5899 * t9432 * t32946 * t3450;
    let t148388 = t23657 * t2185 * t32946 * t27152;
    let t148392 = t89 * t28 * t32709 * t3408;
    (t148381, t148385, t148388, t148392)
}
