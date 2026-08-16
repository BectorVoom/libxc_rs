//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1024/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1024(t12393: f64, t422: f64, t12295: f64, t12292: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64) -> (f64, f64) {
    let t12395 = 0.621814e-1_f64 * t12393 * t422;
    let t12397 = 0.53272592592592592592e-1_f64 * t12295;
    let t12408 = -t12397 + 0.2283111111111111111e-1_f64 * t12297 + 0.11415555555555555555e-1_f64 * t12299 - 0.34246666666666666665e-1_f64 * t12301 - 0.17123333333333333333e-1_f64 * t12303 + 0.19025925925925925925e-1_f64 * t12307 - 0.68493333333333333331e-1_f64 * t12310 - 0.34246666666666666665e-1_f64 * t12292 + 0.10274e0_f64 * t12314 + 0.10274e0_f64 * t12317 + 0.17123333333333333333e-1_f64 * t12320;
    (t12395, t12408)
}
