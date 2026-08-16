//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 246/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk246(t668: f64, t703: f64, t505: f64, t420: f64, t701: f64, t699: f64) -> (f64, f64, f64, f64) {
    let t704 = t703 * t668;
    let t705 = t704 * t505;
    let t706 = t420 * t705;
    let t707 = t701 * t706;
    let t709 = t699 + 0.6384360837962962963e-2_f64 * t707;
    (t704, t705, t707, t709)
}
