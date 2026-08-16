//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 487/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk487(t3494: f64, t439: f64, t3356: f64, t3413: f64, t1178: f64, t447: f64, t1175: f64, t300: f64, t1203: f64, t1208: f64, t487: f64, t1204: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3495 = 1.0_f64 / t3494;
    let t3496 = t439 * t3495;
    let t3503 = 0.40256666666666666667e0_f64 * t3356;
    let t3510 = 0.137975e0_f64 * t3413;
    let t3519 = t1178 * t1178;
    let t3520 = 1.0_f64 / t3519;
    let t3521 = t439 * t3520;
    let t3522 = t447 * t447;
    let t3523 = 1.0_f64 / t3522;
    let t3531 = t300 * t1175;
    let t3546 = 0.11111111111111111111e-1_f64 * t3356;
    let t3555 = t1203 * t1208;
    let t3556 = t3555 * t487;
    let t3561 = t1204 * t487;
    (t3495, t3496, t3503, t3510, t3520, t3521, t3523, t3531, t3546, t3555, t3556, t3561)
}
