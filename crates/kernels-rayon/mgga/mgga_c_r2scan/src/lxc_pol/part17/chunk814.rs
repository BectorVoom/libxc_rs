//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 814/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk814(t44: f64, t6959: f64, t2999: f64, t4938: f64, t1361: f64, t3002: f64, t1216: f64, t4911: f64, t1217: f64, t2466: f64, t415: f64, t48: f64, t3007: f64, t4948: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t8560 = 0.21687162600603479684e-1_f64 * t6959;
    let t8561 = t4938 * t2999;
    let t8566 = t1361 * t3002;
    let t8571 = -2.0_f64 * t1216 - 6.0_f64 * t4911;
    let t8575 = piecewise3(t45, 0.0_f64, -8.0_f64 / 27.0_f64 * t8561 * t415 + 16.0_f64 / 9.0_f64 * t2466 * t1217 + 4.0_f64 / 9.0_f64 * t8566 * t415 + 4.0_f64 / 3.0_f64 * t48 * t8571);
    let t8576 = t4948 * t3007;
    (t8560, t8571, t8575, t8576)
}
