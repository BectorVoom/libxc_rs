//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1048/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1048<F: Float>(t10762: F, t10776: F, t10785: F, t10836: F, t1264: F, t1276: F, t172: F, t184: F, t2127: F, t3244: F, t3248: F, t3263: F, t3281: F, t4026: F, t4031: F, t4048: F, t4059: F, t6628: F, t739: F, t741: F, t755: F, t8780: F, t8802: F) -> (F,) {
    let t10839 = 7.0 / 2.0 * t4048 * t3263 - t8802 * t8780 - t10776 * t3263 / 4.0 - 6.0 * t6628 * t4031 * t739 + 4.0 * t2127 * t1264 * t3244 - t3248 * t10785 / 2.0 + 2.0 * t2127 * t4026 * t739 - t741 * t10762 + 2.0 * t10762 * t184 + 2.0 * t4026 * t755 + 4.0 * t3244 * t1276 + 4.0 * t1264 * t3281 + 2.0 * t739 * t4059 + 2.0 * t172 * t10836;
    (t10839,)
}
