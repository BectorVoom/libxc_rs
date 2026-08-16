//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2657/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2657(t20433: f64, t3866: f64, t16336: f64, t6431: f64, t1831: f64, t57021: f64, t53945: f64, t6396: f64, t12283: f64, t20450: f64, t16233: f64, t19871: f64, t19873: f64, t19876: f64, t20000: f64, t3805: f64, t40192: f64, t5246: f64, t5248: f64, t5250: f64, t5303: f64, t53928: f64, t56685: f64, t56687: f64, t56878: f64, t57081: f64, t57568: f64, t74090: f64, t74120: f64) -> f64 {
    let t74256 = t3866 * t20433;
    let t74258 = t16336 * t6431;
    let t74260 = t57021 * t1831;
    let t74274 = t53945 * t6396;
    let t74276 = t12283 * t20450;
    let t74286 = 35.0_f64 / 192.0_f64 * t74256 + 7.0_f64 / 384.0_f64 * t74258 + 7.0_f64 / 384.0_f64 * t74260 + t56878 * t5303 / 256.0_f64 + t16233 * t3805 * t74120 * t40192 / 128.0_f64 - 3.0_f64 / 512.0_f64 * t16233 * t5248 * t19871 * t57568 + 7.0_f64 / 192.0_f64 * t56685 - 7.0_f64 / 384.0_f64 * t56687 + t53928 - 7.0_f64 / 192.0_f64 * t74274 + 35.0_f64 / 384.0_f64 * t74276 + 3.0_f64 / 512.0_f64 * t19876 * t19873 - 3.0_f64 / 512.0_f64 * t57081 * t20000 + t5246 * t5248 * t74090 * t5250 / 1536.0_f64;
    t74286
}
