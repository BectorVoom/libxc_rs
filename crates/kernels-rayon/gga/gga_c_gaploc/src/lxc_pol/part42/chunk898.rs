//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 898/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk898(t46094: f64, t6710: f64, t6711: f64, t37679: f64, t6590: f64, t10370: f64, t10615: f64, t1457: f64, t44294: f64, t447: f64, t204: f64, t2476: f64) -> (f64, f64, f64, f64, f64) {
    let t46097 = 0.43710935587469654631e2_f64 * t6710 * t6711 * t46094;
    let t46098 = t37679 * t6590;
    let t46102 = 0.50050685932590597338e1_f64 * t10615 * t1457 * t10370;
    let t46103 = t44294 * t447;
    let t46106 = 0.46011511144704899612e1_f64 * t2476 * t204 * t46103;
    (t46097, t46098, t46102, t46103, t46106)
}
