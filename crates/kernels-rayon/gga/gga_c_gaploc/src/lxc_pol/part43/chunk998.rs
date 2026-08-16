//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 998/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk998(t13728: f64, t4614: f64, t597: f64, t1445: f64, t46915: f64, t574: f64, t1: f64, t106: f64, t13749: f64, t192: f64, t536: f64, t40192: f64) -> (f64, f64, f64, f64) {
    let t47902 = t597 * t4614 * t13728;
    let t47912 = 0.46011511144704899612e1_f64 * t574 * t1445 * t46915;
    let t47918 = t13749 * t1 * t106 * t192;
    let t47920 = 0.35750489951850426669e0_f64 * t536 * t47918;
    let t47925 = 0.38342925953920749677e0_f64 * t40192;
    (t47902, t47912, t47920, t47925)
}
