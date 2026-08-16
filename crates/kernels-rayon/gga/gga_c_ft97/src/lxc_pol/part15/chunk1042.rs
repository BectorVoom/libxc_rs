//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1042/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1042(t59354: f64, t59364: f64, t86202: f64, t86205: f64, t86208: f64, t86211: f64, t86214: f64, t86217: f64, t86220: f64, t86223: f64, t86226: f64, t86232: f64, t86236: f64, t86240: f64) -> f64 {
    let t86453 = 2.0_f64 / 9.0_f64 * t86202 + t86205 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t86208 - 4.0_f64 / 27.0_f64 * t86211 + 2.0_f64 / 9.0_f64 * t86214 + 20.0_f64 / 81.0_f64 * t86217 - 10.0_f64 / 27.0_f64 * t86220 + 4.0_f64 / 3.0_f64 * t86223 + 4.0_f64 / 3.0_f64 * t86226 + t59354 - t59364 - 6.0_f64 * t86232 - 4.0_f64 / 3.0_f64 * t86236 - t86240 / 9.0_f64;
    t86453
}
