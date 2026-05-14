//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1370/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1370<F: Float>(t113: F, t2719: F, t494: F, t6085: F, t6086: F, t20313: F, t24210: F, t25951: F, t25955: F, t25959: F, t25964: F, t25970: F, t25974: F, t25979: F, t25981: F, t25984: F, t25990: F, t2721: F, t5109: F, t6132: F, t6487: F, t7338: F, t8029: F) -> (F,) {
    let t25997 = t2719 * t494 * t113;
    let t25999 = t6085 * t6086 * t25997;
    let t26001 = -0.98781737744032673978e-1 * t25951 - 0.13002332610081402845e0 * t6487 * t2721 + 0.20803732176130244552e1 * t25955 + 0.10401866088065122276e1 * t25959 - 0.11708928647259339622e0 * t25964 - 0.69861909304693186866e-1 * t25970 - 0.20958572791407956061e0 * t25974 - t25979 - 0.52396431978519890151e-1 * t25981 + 0.1047928639570397803e0 * t25984 - 0.7801399566048841707e0 * t8029 * t5109 * t24210 + 0.20803732176130244552e1 * t25990 - 0.78013995660488417067e0 * t6132 * t5109 * t7338 * t20313 + 0.34930954652346593433e-1 * t25999;
    (t26001,)
}
