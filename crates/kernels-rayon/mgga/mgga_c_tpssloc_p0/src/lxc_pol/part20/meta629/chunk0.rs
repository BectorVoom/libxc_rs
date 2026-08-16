//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2282/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2282(t47160: f64, t41291: f64, t12932: f64, t2427: f64, t13133: f64, t2430: f64, t145: f64, t185: f64, t46191: f64, t45872: f64, t707: f64, t12886: f64, t706: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47161 = 3.0_f64 * t47160;
    let t47162 = 12.0_f64 * t41291;
    let t47163 = t2427 * t12932;
    let t47164 = 24.0_f64 * t47163;
    let t47165 = t13133 * t2430;
    let t47166 = 24.0_f64 * t47165;
    let t47168 = t145 * t46191 * t185;
    let t47171 = 4.0_f64 * t707 * t185 * t45872;
    let t47172 = t706 * t12886;
    (t47161, t47162, t47164, t47166, t47168, t47171, t47172)
}
