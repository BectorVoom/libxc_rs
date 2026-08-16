//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 485/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk485(t292: f64, t2735: f64, t817: f64, t2689: f64, t2691: f64, t2693: f64, t2720: f64, t2727: f64, t285: f64, t800: f64) -> (f64, f64) {
    let t293 = 0.1e-59_f64 < t292;
    let t2736 = t817 * t2735;
    let t2739 = piecewise3(t293, -4.0_f64 * t2691 * t2693 + 2.0_f64 * t2720 * t800 + 2.0_f64 * t2727 * t285 - t2736 * t285 + 2.0_f64 * t2689, 0.0_f64);
    (t2736, t2739)
}
