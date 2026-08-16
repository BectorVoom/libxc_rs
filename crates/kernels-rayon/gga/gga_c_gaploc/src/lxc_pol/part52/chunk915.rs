//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 915/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk915(t46421: f64, t1: f64, t37573: f64, t1415: f64, t2413: f64, t10533: f64, t20796: f64, t46094: f64, t10532: f64, t46115: f64, t13371: f64, t4614: f64, t574: f64) -> (f64, f64, f64, f64, f64) {
    let t46422 = 0.14896037479937677779e-1_f64 * t46421;
    let t46423 = t37573 * t1;
    let t46424 = t1415 * t46423;
    let t46426 = 0.10725146985555128001e1_f64 * t46424 * t2413;
    let t46432 = 0.27606906686822939767e2_f64 * t20796 * t10533 * t46094;
    let t46435 = 0.55213813373645879534e2_f64 * t10532 * t10533 * t46115;
    let t46447 = 0.61348681526273199483e1_f64 * t574 * t4614 * t13371;
    (t46422, t46426, t46432, t46435, t46447)
}
