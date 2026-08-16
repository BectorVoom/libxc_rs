//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1007/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1007(t378: f64, t85538: f64, t92: f64, t58969: f64, t73956: f64, t73958: f64, t73983: f64, t85518: f64, t85522: f64, t85526: f64, t85529: f64, t85533: f64, t85536: f64) -> (f64, f64) {
    let t85540 = t92 * t378 * t85538;
    let t85542 = -8.0_f64 / 9.0_f64 * t58969 - 8.0_f64 / 3.0_f64 * t73956 + 8.0_f64 / 9.0_f64 * t73958 + 40.0_f64 / 9.0_f64 * t85518 + 40.0_f64 / 81.0_f64 * t73983 - 20.0_f64 / 9.0_f64 * t85522 - 8.0_f64 * t85526 + 8.0_f64 * t85529 - 2.0_f64 / 3.0_f64 * t85533 - 8.0_f64 / 9.0_f64 * t85536 + 8.0_f64 * t85540;
    (t85540, t85542)
}
