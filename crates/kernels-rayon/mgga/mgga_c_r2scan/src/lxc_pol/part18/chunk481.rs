//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 481/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk481(t44: f64, t51: f64, t1216: f64, t2509: f64, t2512: f64, t415: f64, t1224: f64, t893: f64, t35: f64, t476: f64, t419: f64, zeta_threshold: f64) -> (f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t2516 = piecewise3(t45, 0.0_f64, -2.0_f64 / 9.0_f64 * t2509 * t415 + 4.0_f64 / 3.0_f64 * t2512 * t1216);
    let t2517 = t1224 * t893;
    let t2520 = t476 * t35;
    let t2524 = piecewise3(t52, 0.0_f64, -2.0_f64 / 9.0_f64 * t2517 * t419 - 4.0_f64 / 3.0_f64 * t2520 * t1216);
    let t2526 = t2516 / 2.0_f64 + t2524 / 2.0_f64;
    (t2517, t2526)
}
