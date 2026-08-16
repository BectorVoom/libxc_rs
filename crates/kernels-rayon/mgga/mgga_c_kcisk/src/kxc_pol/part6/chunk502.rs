//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 502/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk502(t642: f64, t695: f64, t1755: f64, t654: f64, t1906: f64, t751: f64, t724: f64, t574: f64, t725: f64, t140: f64, t430: f64, t728: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5193 = t642 * t695;
    let t5203 = t654 * t1755;
    let t5217 = 1.0_f64 / t1906 / t751;
    let t5218 = t724 * t5217;
    let t5231 = t725 * t574;
    let t5242 = 0.88437037037037037037e-2_f64 * t140 * t430 * t728;
    (t5193, t5203, t5217, t5218, t5231, t5242)
}
