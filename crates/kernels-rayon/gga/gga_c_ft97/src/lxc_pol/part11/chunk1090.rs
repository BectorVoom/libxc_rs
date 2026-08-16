//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1090/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1090(t41950: f64, t41947: f64, t41953: f64, t41957: f64, t41960: f64, t41964: f64, t41969: f64, t41973: f64, t41978: f64, t41981: f64, t42053: f64, t42057: f64, t42233: f64, t42236: f64, t42240: f64) -> f64 {
    let t42759 = 280.0_f64 / 81.0_f64 * t41950;
    let t42772 = 4.0_f64 / 3.0_f64 * t41947 + t42759 - 15.0_f64 / 16.0_f64 * t42053 - 3.0_f64 / 4.0_f64 * t42057 + t42233 / 2.0_f64 - t42236 + 9.0_f64 / 4.0_f64 * t42240 - 8.0_f64 / 9.0_f64 * t41953 - 16.0_f64 / 27.0_f64 * t41957 - 16.0_f64 / 9.0_f64 * t41960 + 40.0_f64 / 81.0_f64 * t41964 + 40.0_f64 / 9.0_f64 * t41969 - t41973 / 3.0_f64 - 36.0_f64 * t41978 + 112.0_f64 / 81.0_f64 * t41981;
    t42772
}
