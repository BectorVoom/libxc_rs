//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1115/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1115(t5: f64, t33118: f64, t8513: f64, t31004: f64, t31010: f64, t31017: f64, t31022: f64, t33103: f64, t33107: f64, t33111: f64, t33115: f64, t8309: f64, t112: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t33119 = t8513 * t33118;
    let t33123 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t33103 * t8309 - 5.0_f64 / 24.0_f64 * t31004 * t33107 - 5.0_f64 / 36.0_f64 * t31010 * t33111 + 5.0_f64 / 72.0_f64 * t31017 * t33115 + 5.0_f64 / 72.0_f64 * t31022 * t33119);
    let t33124 = t33123 * t112;
    (t33119, t33123, t33124)
}
