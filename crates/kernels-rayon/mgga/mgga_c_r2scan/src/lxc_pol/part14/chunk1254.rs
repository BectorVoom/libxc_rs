//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1254/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1254(t37390: f64, t39290: f64, t39309: f64, t41223: f64, t41225: f64, t41227: f64, t41230: f64, t41233: f64, t41236: f64, t41240: f64, t41243: f64, t41247: f64, t41251: f64, t41254: f64, t41256: f64) -> f64 {
    let t42175 = -t41223 + 0.3842256877732895568e-2_f64 * t37390 + t41225 + t41227 + 0.60975299583150056624e-3_f64 * t39290 - t41230 - t41233 - t41236 + 0.30487649791575028312e-3_f64 * t39309 - t41240 + t41243 + t41247 - t41251 - t41254 - t41256;
    t42175
}
