//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2419/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2419(t39382: f64, t761: f64, t2531: f64, t9713: f64, t39302: f64, t2371: f64, t9716: f64, t2447: f64, t32: f64, t31: f64, t717: f64, t607: f64, t707: f64, t9862: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41258 = 0.91082604192152556044e5_f64 * t761 * t39382;
    let t41259 = t2531 * t9713;
    let t41262 = 0.5848223622634646207e0_f64 * t761 * t39302;
    let t41274 = t9716 * t2371;
    let t41279 = t32 * t2447;
    let t41284 = t31 * t717;
    let t41291 = t707 * t9862 * t607;
    (t41258, t41259, t41262, t41274, t41279, t41284, t41291)
}
