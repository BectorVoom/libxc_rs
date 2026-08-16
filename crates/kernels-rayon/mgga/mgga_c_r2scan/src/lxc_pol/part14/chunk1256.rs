//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1256/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1256(t40312: f64, t37431: f64, t37438: f64, t40294: f64, t40303: f64, t40305: f64, t40308: f64, t40315: f64, t40319: f64, t41280: f64, t41283: f64, t41285: f64, t41286: f64, t41289: f64, t41291: f64) -> f64 {
    let t42187 = 0.1440846329149835838e-2_f64 * t40312;
    let t42192 = -t41280 + t41283 - t41285 - 0.72042316457491791901e-3_f64 * t40294 - 0.3842256877732895568e-2_f64 * t40303 + 0.92232789896410962669e-3_f64 * t40305 + 0.72042316457491791901e-3_f64 * t40308 + t42187 - 0.86737941314158990616e-4_f64 * t40315 - 0.20496175532535769482e-3_f64 * t40319 - t41286 + t41289 + t41291 - 0.2881692658299671676e-2_f64 * t37431 + 0.40992351065071538965e-3_f64 * t37438;
    t42192
}
