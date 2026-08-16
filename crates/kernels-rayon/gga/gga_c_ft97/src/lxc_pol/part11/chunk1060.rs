//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1060/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1060(t2330: f64, t2464: f64, t263: f64, t41403: f64, t41405: f64, t41411: f64, t41414: f64, t41417: f64, t41419: f64, t41421: f64, t41821: f64, t41875: f64, t41929: f64, t41983: f64, t661: f64, t771: f64, t9511: f64, t9512: f64, t9780: f64) -> f64 {
    let t41988 = -48.0_f64 * t41403 + 48.0_f64 * t41405 + 48.0_f64 * t41411 - 72.0_f64 * t41414 + 24.0_f64 * t41417 - 12.0_f64 * t41419 - 8.0_f64 * t41421 - 3.0_f64 * t2330 * t9780 * t263 - 3.0_f64 * t9511 * t2464 * t263 - 4.0_f64 * t9512 * t771 - t661 * (t41821 + t41875 + t41929 + t41983) * t263;
    t41988
}
