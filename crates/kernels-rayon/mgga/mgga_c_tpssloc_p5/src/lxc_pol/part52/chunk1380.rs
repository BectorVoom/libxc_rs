//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1380/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1380(t24932: f64, t7468: f64, t27888: f64, t26003: f64, t7266: f64, t120005: f64, t120008: f64, t120019: f64, t120020: f64, t120022: f64, t120027: f64, t120029: f64, t120040: f64, t120044: f64, t123062: f64, t672: f64) -> f64 {
    let t123138 = t24932 * t7468;
    let t123140 = t27888 * t7468;
    let t123142 = t7266 * t26003;
    let t123151 = -2.0_f64 * t123062 * t672 - t120005 - t120008 - t120019 - 2.0_f64 * t120020 - 2.0_f64 * t120022 - 2.0_f64 * t120027 - 2.0_f64 * t120029 - 2.0_f64 * t120040 + t120044 - 2.0_f64 * t123138 - 2.0_f64 * t123140 - 2.0_f64 * t123142;
    t123151
}
