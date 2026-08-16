//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1038/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1038(t38771: f64, t57620: f64, t73343: f64, t73358: f64, t73405: f64, t86246: f64, t86250: f64, t86254: f64, t86258: f64, t86264: f64, t86268: f64, t86274: f64, t86278: f64, t86281: f64, t86284: f64) -> f64 {
    let t86386 = 40.0_f64 / 27.0_f64 * t86246 + 8.0_f64 / 3.0_f64 * t86250 - 80.0_f64 / 243.0_f64 * t86254 - t86258 / 9.0_f64 - 8.0_f64 / 3.0_f64 * t73343 + t38771 - 4.0_f64 / 9.0_f64 * t73358 + 8.0_f64 / 3.0_f64 * t86264 + 2.0_f64 / 3.0_f64 * t86268 + 16.0_f64 / 9.0_f64 * t57620 - 8.0_f64 / 27.0_f64 * t73405 + 8.0_f64 * t86274 + 2.0_f64 * t86278 + 3.0_f64 / 4.0_f64 * t86281 - t86284 / 3.0_f64;
    t86386
}
