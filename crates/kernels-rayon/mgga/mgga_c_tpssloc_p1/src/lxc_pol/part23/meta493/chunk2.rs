//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1514/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1514(t1363: f64, t1367: f64, t19904: f64, t20433: f64, t3870: f64, t40070: f64, t5240: f64, t53901: f64, t6427: f64, t6431: f64, t74191: f64, t74212: f64, t74214: f64, t74217: f64, t74228: f64, t74256: f64, t79921: f64, t79984: f64, t80021: f64, t820: f64) -> f64 {
    let t80330 = -35.0_f64 / 96.0_f64 * t74191 + 595.0_f64 / 648.0_f64 * t53901 + 7.0_f64 / 384.0_f64 * t74212 + 7.0_f64 / 192.0_f64 * t74214 + 7.0_f64 / 384.0_f64 * t74217 - 7.0_f64 / 192.0_f64 * t74228 + 35.0_f64 / 128.0_f64 * t1363 * t40070 * t820 * t80021 + 5.0_f64 / 256.0_f64 * t1363 * t3870 * t820 * t79921 + 5.0_f64 / 128.0_f64 * t19904 * t6427 - t1363 * t1367 * t820 * t79984 / 768.0_f64 - 5.0_f64 / 32.0_f64 * t5240 * t20433 - t19904 * t6431 / 128.0_f64 + 35.0_f64 / 48.0_f64 * t74256;
    t80330
}
