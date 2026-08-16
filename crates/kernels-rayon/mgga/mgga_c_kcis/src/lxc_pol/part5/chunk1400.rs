//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1400/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1400(t12827: f64, t12840: f64, t18069: f64, t18192: f64, t23073: f64, t23077: f64, t23080: f64, t23083: f64, t23088: f64, t23093: f64, t23098: f64, t23103: f64, t23107: f64, t4439: f64, t6173: f64) -> f64 {
    let t23113 = t18069 / 162.0_f64 + t4439 * t23073 / 72.0_f64 - t4439 * t23077 / 576.0_f64 - t4439 * t23080 / 288.0_f64 + t4439 * t23083 / 432.0_f64 + t4439 * t23088 / 288.0_f64 - t4439 * t23093 / 576.0_f64 + t12840 + t4439 * t23098 / 144.0_f64 + t4439 * t23103 / 144.0_f64 - t4439 * t23107 / 216.0_f64 - t12827 / 2592.0_f64 + t18192 * t6173 / 108.0_f64;
    t23113
}
