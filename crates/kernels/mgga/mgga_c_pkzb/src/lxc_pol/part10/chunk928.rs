//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 928/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk928<F: Float>(t1692: F, t2719: F, t1020: F, t1535: F, t1634: F, t2714: F, t2718: F, t5025: F, t5028: F, t5040: F, t5066: F, t5069: F, t5073: F, t5082: F, t5186: F, t5324: F, t5333: F, t5338: F, t5344: F, t7043: F, t7045: F, t7048: F, t7049: F, t7050: F, t7051: F) -> (F,) {
    let t7209 = t2719 * t1692;
    let t7215 = -3.0 * t1020 * t1535 * t5082 + 6.0 * t1634 * t2714 * t2718 + 6.0 * t2718 * t7209 + t5025 + t5028 + t5040 + t5066 - t5069 - t5073 + t5186 - t5324 + t5333 - t5338 - t5344 - t7043 + t7045 + t7048 + t7049 - t7050 - t7051;
    (t7215,)
}
