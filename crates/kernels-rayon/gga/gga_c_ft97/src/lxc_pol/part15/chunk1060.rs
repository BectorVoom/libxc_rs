//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1060/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1060(t140: f64, t86867: f64, t4431: f64, t4822: f64, t11761: f64, t12791: f64, t17338: f64, t24: f64, t4714: f64, t49921: f64, t586: f64, t62745: f64, t62751: f64, t78179: f64, t78181: f64, t78183: f64, t78185: f64, t78188: f64, t78242: f64, t78247: f64, t78249: f64, t92: f64) -> (f64, f64, f64) {
    let t141 = 0.1e-59_f64 < t140;
    let t86868 = piecewise3(t141, t86867, 0.0_f64);
    let t86876 = t4822 * t4431;
    let t86891 = -t92 * t24 * t586 * t86868 - 8.0_f64 * t11761 * t17338 * t4822 * t4714 - 8.0_f64 * t11761 * t12791 * t86876 + 4.0_f64 / 9.0_f64 * t78179 - 16.0_f64 / 9.0_f64 * t78181 + 8.0_f64 / 3.0_f64 * t78183 - 8.0_f64 * t78185 + 40.0_f64 / 81.0_f64 * t78188 + 112.0_f64 / 27.0_f64 * t49921 - 8.0_f64 / 3.0_f64 * t62745 + 16.0_f64 / 3.0_f64 * t62751 - 8.0_f64 / 3.0_f64 * t78242 + 8.0_f64 / 9.0_f64 * t78247 - 4.0_f64 / 3.0_f64 * t78249;
    (t86868, t86876, t86891)
}
