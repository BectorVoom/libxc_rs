//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1077/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1077<F: Float>(t2492: F, t4300: F, t941: F, t11283: F, t946: F, t238: F, t4310: F, t800: F, t1386: F, t3487: F, t242: F, t4314: F, t4283: F, t929: F, t11280: F, t341: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11296 = t2492 * t4300;
    let t11297 = t11296 * t941;
    let t11299 = t946 * t11283;
    let t11304 = t238 * t800 * t4310;
    let t11306 = t1386 * t3487;
    let t11308 = t238 * t242 * t11306;
    let t11311 = t238 * t800 * t4314;
    let t11313 = t929 * t4283;
    let t11315 = t238 * t242 * t11313;
    let t11317 = t341 * t11280;
    (t11296, t11297, t11299, t11304, t11306, t11308, t11311, t11313, t11315, t11317)
}
