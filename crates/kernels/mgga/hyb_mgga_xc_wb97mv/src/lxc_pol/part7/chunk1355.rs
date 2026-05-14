//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1355/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1355<F: Float>(t2856: F, t4550: F, t2848: F, t4541: F, t1112: F, t1117: F, t1148: F, t11741: F, t11748: F, t11752: F, t11753: F, t11756: F, t24761: F, t2839: F, t2849: F, t2860: F, t2869: F, t2873: F, t3803: F, t4554: F, t4581: F, t4591: F, t4594: F, t4597: F, t4603: F, t505: F, t529: F, t7907: F) -> (F,) {
    let t33244 = t2856 * t4550;
    let t33276 = t2848 * t4541;
    let t33280 = -336.0 * t529 * t7907 * t4554 * t2849 - 4.0 * t1117 * t11752 * t2873 + 12.0 * t1117 * t33244 * t2849 - 4.0 * t1117 * t4603 * t2869 - 36.0 * t1148 * t11741 * t2873 - 36.0 * t1148 * t4597 * t2869 + 42.0 * t529 * t11748 * t2873 + 360.0 * t24761 * t4591 * t2839 + 1260.0 * t2860 * t4581 * t2839 - 180.0 * t2860 * t4597 * t2839 + 1260.0 * t2860 * t33276 * t2849 + 30.0 * t2860 * t4594 * t2869 + 2.0 * t505 * t4603 * t2873 + 4.0 * t1112 * t11756 - 8.0 * t3803 * t11753;
    (t33280,)
}
