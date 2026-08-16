//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 897/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk897(t37292: f64, t1766: f64, t473: f64, t8336: f64, t91: f64, t1767: f64, t1808: f64, t8345: f64, t37254: f64, t37257: f64, t37261: f64, t37266: f64, t37271: f64, t37275: f64, t37277: f64, t37281: f64, t37285: f64, t37289: f64, t37296: f64, t37300: f64) -> (f64, f64, f64) {
    let t38392 = 280.0_f64 / 81.0_f64 * t37292;
    let t38397 = t91 * t1766 * t8336 * t473;
    let t38400 = t91 * t8345 * t1767 * t1808;
    let t38402 = 8.0_f64 / 3.0_f64 * t37254 - 8.0_f64 * t37257 + 4.0_f64 / 3.0_f64 * t37261 + 8.0_f64 / 3.0_f64 * t37266 - 8.0_f64 / 9.0_f64 * t37271 + 8.0_f64 * t37275 + 8.0_f64 / 3.0_f64 * t37277 + 4.0_f64 / 3.0_f64 * t37281 + 2.0_f64 * t37285 + 8.0_f64 * t37289 + t38392 - 8.0_f64 * t37296 - 8.0_f64 * t37300 - t38397 + 9.0_f64 / 4.0_f64 * t38400;
    (t38397, t38400, t38402)
}
