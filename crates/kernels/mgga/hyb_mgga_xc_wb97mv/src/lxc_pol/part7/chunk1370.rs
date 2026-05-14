//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1370/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1370<F: Float>(t11689: F, t653: F, t9852: F, t10172: F, t10190: F, t28042: F, t28048: F, t28053: F, t28410: F, t28617: F, t28621: F, t28677: F, t28686: F, t28755: F, t33143: F, t33147: F, t33151: F, t33155: F, t33167: F, t33173: F, t33177: F, t33843: F) -> (F,) {
    let t33850 = t11689 * t653 * t9852;
    let t33879 = 0.79999999999999999999e0 * t28621 * t33155 - 0.21333333333333333333e-2 * t28053 * t33850 + 0.32e-2 * t28755 * t33843 - 0.16e-1 * t28042 * t33850 + 12.0 * t28617 * t33143 - 0.168e2 * t28410 * t33147 + 0.192e-1 * t28048 * t33843 - 12.0 * t28617 * t33151 + 0.168e2 * t28410 * t33155 - 0.192e-1 * t28048 * t33850 - 0.6144e-5 * t28686 * t33173 - 0.17066666666666666667e-4 * t10172 * t33177 + 0.21333333333333333334e-2 * t28053 * t33167 + 0.224e-1 * t28677 * t33843 - 0.512e-4 * t10190 * t33177;
    (t33879,)
}
