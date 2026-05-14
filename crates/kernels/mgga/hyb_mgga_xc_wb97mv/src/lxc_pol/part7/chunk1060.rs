//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1060/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1060<F: Float>(t2217: F, t4170: F, t790: F, t10995: F, t795: F, t238: F, t4180: F, t800: F, t1323: F, t3326: F, t242: F, t4184: F, t4153: F, t778: F, t10976: F, t226: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11008 = t2217 * t4170;
    let t11009 = t11008 * t790;
    let t11011 = t795 * t10995;
    let t11016 = t238 * t800 * t4180;
    let t11018 = t1323 * t3326;
    let t11020 = t238 * t242 * t11018;
    let t11023 = t238 * t800 * t4184;
    let t11025 = t778 * t4153;
    let t11027 = t238 * t242 * t11025;
    let t11029 = t226 * t10976;
    (t11008, t11009, t11011, t11016, t11018, t11020, t11023, t11025, t11027, t11029)
}
