//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1359/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1359(t15936: f64, t6028: f64, t6027: f64, t17333: f64, t4261: f64, t17428: f64, t17431: f64, t17434: f64, t17437: f64, t17439: f64, t17441: f64, t17444: f64, t17447: f64, t17451: f64, t17455: f64, t17458: f64, t17461: f64, t17465: f64, t17468: f64, t17472: f64, t17475: f64) -> (f64, f64, f64) {
    let t17477 = t6028 * t15936;
    let t17478 = t6027 * t17477;
    let t17480 = t4261 * t17333;
    let t17481 = t6027 * t17480;
    let t17483 = t17428 / 108.0_f64 - t17431 / 12.0_f64 + t17434 / 36.0_f64 - t17437 / 96.0_f64 + t17439 / 128.0_f64 - t17441 / 96.0_f64 - t17444 / 288.0_f64 + t17447 / 96.0_f64 - t17451 / 8.0_f64 + t17455 / 288.0_f64 + t17458 / 192.0_f64 - t17461 / 24.0_f64 + 3.0_f64 / 128.0_f64 * t17465 + t17468 / 192.0_f64 + t17472 / 864.0_f64 + 2.0_f64 / 9.0_f64 * t17475 + t17478 / 8.0_f64 + t17481 / 6.0_f64;
    (t17478, t17481, t17483)
}
