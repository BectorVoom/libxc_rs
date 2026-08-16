//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 702/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk702(t1429: f64, t4803: f64, t15: f64, t25: f64, t26: f64, t1436: f64, t444: f64, t1435: f64, t1440: f64, t27: f64, t1419: f64, t1426: f64, t1432: f64, t16: f64, t23: f64, t434: f64, t441: f64, t4784: f64, t4796: f64, t4800: f64, t7: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4804 = t1429 - t4803;
    let t4805 = 3.0_f64 * t4804;
    let t4806 = t15 * t4805;
    let t4809 = t26 * t25;
    let t4810 = 1.0_f64 / t4809;
    let t4811 = t1436 * t444;
    let t4812 = t4810 * t4811;
    let t4815 = t1435 * t444;
    let t4816 = t4815 * t1440;
    let t4819 = -t4805;
    let t4820 = t27 * t4819;
    let t4823 = -1232.0_f64 / 27.0_f64 * t4784 * t16 + 440.0_f64 / 9.0_f64 * t1419 * t441 - 80.0_f64 / 9.0_f64 * t434 * t1426 - 40.0_f64 / 3.0_f64 * t434 * t1432 - 10.0_f64 / 27.0_f64 * t7 * t4796 + 10.0_f64 / 3.0_f64 * t7 * t4800 + 5.0_f64 / 3.0_f64 * t7 * t4806 - 10.0_f64 / 27.0_f64 * t23 * t4812 + 10.0_f64 / 3.0_f64 * t23 * t4816 + 5.0_f64 / 3.0_f64 * t23 * t4820;
    (t4804, t4805, t4806, t4810, t4811, t4812, t4816, t4819, t4820, t4823)
}
