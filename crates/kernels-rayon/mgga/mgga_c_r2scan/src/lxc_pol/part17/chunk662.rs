//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 662/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk662(t453: f64, t4811: f64, t1379: f64, t81: f64, t76: f64, t1384: f64, t1481: f64, t28: f64, t14: f64, t1467: f64, t400: f64, t1485: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4812 = t4811 * t453;
    let t4816 = 1.0_f64 / t1379 / t81;
    let t4817 = t76 * t4816;
    let t4818 = t4811 * t1384;
    let t4822 = 1.0_f64 / t1481 / t28;
    let t4823 = t14 * t4822;
    let t4824 = t1467 * t400;
    let t4825 = t4824 * t1485;
    (t4812, t4816, t4817, t4818, t4823, t4824, t4825)
}
