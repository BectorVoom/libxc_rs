//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 935/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk935<F: Float>(t1264: F, t1276: F, t172: F, t184: F, t2115: F, t2123: F, t2127: F, t2144: F, t2155: F, t3244: F, t3248: F, t3249: F, t3281: F, t6628: F, t739: F, t741: F, t755: F, t8761: F, t8774: F, t8777: F, t8780: F, t8838: F) -> (F,) {
    let t8841 = 7.0 / 2.0 * t2144 * t3249 - t8774 * t3249 / 2.0 - t8777 * t3249 / 4.0 - t3248 * t8780 - 6.0 * t6628 * t1264 * t2123 + 4.0 * t2127 * t3244 * t739 + 2.0 * t2127 * t1264 * t2115 - t741 * t8761 + 2.0 * t8761 * t184 + 4.0 * t3244 * t755 + 2.0 * t1264 * t2155 + 2.0 * t2115 * t1276 + 4.0 * t739 * t3281 + 2.0 * t172 * t8838;
    (t8841,)
}
