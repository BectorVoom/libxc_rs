//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1069/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1069<F: Float>(t10953: F, t10955: F, t11132: F, t11135: F, t11139: F, t11146: F, t11149: F, t11152: F, t11155: F, t11159: F, t11162: F, t2262: F, t2284: F, t2323: F, t3388: F, t3407: F, t6876: F, t6966: F, t9061: F, t9084: F) -> (F,) {
    let t11165 = 0.17315859105681463759e2 * t2323 * t11132 + 0.34631718211362927518e2 * t2323 * t11135 + 0.10254018858216406658e4 * t6966 * t11139 - 4.0 * t9084 * t3388 + 0.64327917994770140268e2 * t9061 * t3407 + 6.0 * t2284 * t11146 - 4.0 * t2262 * t11149 - 0.19298375398431042081e3 * t6876 * t11152 - 2.0 * t2262 * t11155 + 0.32163958997385070134e2 * t2284 * t11159 + 0.64327917994770140268e2 * t2284 * t11162 - t10953 - t10955;
    (t11165,)
}
