//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 973/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk973(t10532: f64, t10533: f64, t46115: f64, t13371: f64, t4614: f64, t574: f64, t1457: f64, t44474: f64, t4540: f64, t13468: f64, t21370: f64, t1445: f64, t44480: f64, t597: f64) -> (f64, f64, f64, f64, f64) {
    let t46435 = 0.55213813373645879534e2_f64 * t10532 * t10533 * t46115;
    let t46447 = 0.61348681526273199483e1_f64 * t574 * t4614 * t13371;
    let t46450 = 0.21450293971110256001e1_f64 * t4540 * t1457 * t44474;
    let t46457 = t21370 * t13468;
    let t46461 = 0.11502877786176224903e2_f64 * t597 * t1445 * t44480;
    (t46435, t46447, t46450, t46457, t46461)
}
