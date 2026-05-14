//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 688/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk688<F: Float>(t1429: F, t4803: F, t15: F, t25: F, t26: F, t1436: F, t444: F, t1435: F, t1440: F, t27: F, t1419: F, t1426: F, t1432: F, t16: F, t23: F, t434: F, t441: F, t4784: F, t4796: F, t4800: F, t7: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4804 = t1429 - t4803;
    let t4805 = 3.0 * t4804;
    let t4806 = t15 * t4805;
    let t4809 = t26 * t25;
    let t4810 = 1.0 / t4809;
    let t4811 = t1436 * t444;
    let t4812 = t4810 * t4811;
    let t4815 = t1435 * t444;
    let t4816 = t4815 * t1440;
    let t4819 = -t4805;
    let t4820 = t27 * t4819;
    let t4823 = -1232.0 / 27.0 * t4784 * t16 + 440.0 / 9.0 * t1419 * t441 - 80.0 / 9.0 * t434 * t1426 - 40.0 / 3.0 * t434 * t1432 - 10.0 / 27.0 * t7 * t4796 + 10.0 / 3.0 * t7 * t4800 + 5.0 / 3.0 * t7 * t4806 - 10.0 / 27.0 * t23 * t4812 + 10.0 / 3.0 * t23 * t4816 + 5.0 / 3.0 * t23 * t4820;
    (t4804, t4805, t4806, t4810, t4811, t4812, t4816, t4819, t4820, t4823)
}
