//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1006/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1006(t1637: f64, t2135: f64, t89: f64, t13208: f64, t1651: f64, t167: f64, t1901: f64, t2075: f64, t2142: f64, t2157: f64, t2185: f64, t2230: f64, t38053: f64, t38057: f64, t39664: f64, t41002: f64, t41019: f64, t446: f64, t569: f64, t574: f64, t605: f64, t609: f64, t616: f64, t7973: f64, t9007: f64, t9304: f64, t9316: f64) -> f64 {
    let t41040 = t89 * t1637 * t2135;
    let t41045 = -80.0_f64 / 243.0_f64 * t446 * t41002 * t167 * t38053 - 2.0_f64 / 3.0_f64 * t446 * t569 * t2230 * t1651 - 4.0_f64 / 9.0_f64 * t446 * t569 * t616 * t7973 - t446 * t569 * t167 * t38057 / 9.0_f64 + 4.0_f64 / 9.0_f64 * t41019 + 2.0_f64 * t446 * t574 * t605 * t2075 * t2157 + 4.0_f64 * t446 * t574 * t2142 * t9316 + 4.0_f64 / 3.0_f64 * t446 * t574 * t605 * t9007 * t609 - 8.0_f64 * t446 * t2185 * t2142 * t9304 + 8.0_f64 / 9.0_f64 * t41040 - 8.0_f64 / 3.0_f64 * t1901 * t13208 * t39664;
    t41045
}
