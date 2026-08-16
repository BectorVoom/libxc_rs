//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 702/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk702<F: Float>(t1429: F, t4803: F, t15: F, t25: F, t26: F, t1436: F, t444: F, t1435: F, t1440: F, t27: F, t1419: F, t1426: F, t1432: F, t16: F, t23: F, t434: F, t441: F, t4784: F, t4796: F, t4800: F, t7: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4804 = t1429 - t4803;
    let t4805 = F::cast_from(3.0_f64) * t4804;
    let t4806 = t15 * t4805;
    let t4809 = t26 * t25;
    let t4810 = F::cast_from(1.0_f64) / t4809;
    let t4811 = t1436 * t444;
    let t4812 = t4810 * t4811;
    let t4815 = t1435 * t444;
    let t4816 = t4815 * t1440;
    let t4819 = -t4805;
    let t4820 = t27 * t4819;
    let t4823 = -F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t4784 * t16 + F::cast_from(440.0_f64) / F::cast_from(9.0_f64) * t1419 * t441 - F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t434 * t1426 - F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t434 * t1432 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t7 * t4796 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t7 * t4800 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7 * t4806 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t23 * t4812 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t23 * t4816 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t23 * t4820;
    (t4804, t4805, t4806, t4810, t4811, t4812, t4816, t4819, t4820, t4823)
}
