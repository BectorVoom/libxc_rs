//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1072/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1072(t42104: f64, t42143: f64, t42191: f64, t42229: f64, t734: f64, t91: f64, t9881: f64, t9968: f64, t2476: f64, t2514: f64, t9890: f64, t41947: f64, t41953: f64, t41957: f64, t41960: f64, t41964: f64, t41969: f64, t41973: f64, t41978: f64, t41981: f64, t42044: f64, t42053: f64, t42057: f64) -> (f64, f64, f64, f64) {
    let t42233 = t91 * t734 * (t42104 + t42143 + t42191 + t42229);
    let t42236 = t91 * t9881 * t9968;
    let t42240 = t91 * t9890 * t2476 * t2514;
    let t42250 = 4.0_f64 / 9.0_f64 * t41947 + t42044 - 5.0_f64 / 16.0_f64 * t42053 - t42057 / 4.0_f64 + t42233 / 6.0_f64 - t42236 / 3.0_f64 + 3.0_f64 / 4.0_f64 * t42240 - 8.0_f64 / 27.0_f64 * t41953 - 16.0_f64 / 81.0_f64 * t41957 - 16.0_f64 / 27.0_f64 * t41960 + 40.0_f64 / 243.0_f64 * t41964 + 40.0_f64 / 27.0_f64 * t41969 - t41973 / 9.0_f64 - 12.0_f64 * t41978 + 112.0_f64 / 243.0_f64 * t41981;
    (t42233, t42236, t42240, t42250)
}
