//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1230/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1230(t41274: f64, t185: f64, t39110: f64, t707: f64, t2447: f64, t32: f64, t2659: f64, t9929: f64, t9932: f64, t31: f64, t717: f64, t9898: f64) -> (f64, f64, f64, f64, f64) {
    let t41275 = 0.70178683471615754484e1_f64 * t41274;
    let t41278 = 4.0_f64 * t707 * t185 * t39110;
    let t41279 = t32 * t2447;
    let t41281 = 72.0_f64 * t41279 * t2659;
    let t41282 = t9929 * t9932;
    let t41283 = 144.0_f64 * t41282;
    let t41284 = t31 * t717;
    let t41286 = 96.0_f64 * t41284 * t9898;
    (t41275, t41278, t41281, t41283, t41286)
}
