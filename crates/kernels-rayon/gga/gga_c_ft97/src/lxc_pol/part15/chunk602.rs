//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 602/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk602(t10397: f64, t170: f64, t328: f64, t8715: f64, t703: f64, t900: f64, t327: f64, t9577: f64, t230: f64, t2938: f64, t9556: f64, t2937: f64, t325: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10797 = 28.0_f64 / 27.0_f64 * t10397;
    let t10838 = 20.0_f64 / 27.0_f64 * t170 * t8715 * t328;
    let t10845 = t703 * t900;
    let t10850 = t327 * t9577;
    let t10864 = t230 * t2938;
    let t10883 = 0.44934037037037037036e0_f64 * t9556;
    let t10904 = 1.0_f64 / t2937 / t325;
    (t10797, t10838, t10845, t10850, t10864, t10883, t10904)
}
