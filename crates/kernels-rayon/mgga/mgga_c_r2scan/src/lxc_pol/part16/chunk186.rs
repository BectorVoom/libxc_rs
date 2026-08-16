//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 186/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk186(t44: f64, t51: f64, t415: f64, t99: f64, t101: f64, t419: f64, zeta_threshold: f64) -> f64 {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t555 = piecewise3(t45, 0.0_f64, 5.0_f64 / 3.0_f64 * t99 * t415);
    let t558 = piecewise3(t52, 0.0_f64, 5.0_f64 / 3.0_f64 * t101 * t419);
    let t560 = t555 / 2.0_f64 + t558 / 2.0_f64;
    t560
}
