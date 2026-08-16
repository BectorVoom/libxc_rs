//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 936/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk936(t12925: f64, t4614: f64, t574: f64, t3133: f64, t4752: f64, t8352: f64, t12887: f64, t1641: f64, t39624: f64, t39626: f64, t39632: f64, t39637: f64, t39642: f64, t39646: f64, t39648: f64, t39650: f64, t471: f64) -> (f64, f64, f64, f64) {
    let t42081 = 0.61348681526273199483e1_f64 * t574 * t4614 * t12925;
    let t42092 = 0.28600391961480341335e1_f64 * t8352 * t4752 * t3133;
    let t42099 = 0.92023022289409799224e1_f64 * t1641 * t12887;
    let t42111 = (21.0_f64 / 512.0_f64 * t39624 + 357.0_f64 / 16384.0_f64 * t39626 - 189.0_f64 / 262144.0_f64 * t39632 + 189.0_f64 / 0.16777216e8_f64 * t39637 - 63.0_f64 / 0.16777216e8_f64 * t39642 + 63.0_f64 / 262144.0_f64 * t39646 - 119.0_f64 / 16384.0_f64 * t39648 - 7.0_f64 / 512.0_f64 * t39650) * t471;
    (t42081, t42092, t42099, t42111)
}
