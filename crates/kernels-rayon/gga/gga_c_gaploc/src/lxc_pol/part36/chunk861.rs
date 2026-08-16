//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 861/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk861(t42071: f64, t1457: f64, t1572: f64, t41865: f64, t12904: f64, t4614: f64, t574: f64, t12925: f64, t12792: f64, t203: f64, t447: f64, t3133: f64, t4752: f64, t8352: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42072 = 0.25561950635947166451e0_f64 * t42071;
    let t42074 = t1572 * t1457 * t41865;
    let t42077 = t574 * t4614 * t12904;
    let t42081 = 0.61348681526273199483e1_f64 * t574 * t4614 * t12925;
    let t42085 = t203 * t12792;
    let t42086 = t42085 * t447;
    let t42092 = 0.28600391961480341335e1_f64 * t8352 * t4752 * t3133;
    (t42072, t42074, t42077, t42081, t42085, t42086, t42092)
}
