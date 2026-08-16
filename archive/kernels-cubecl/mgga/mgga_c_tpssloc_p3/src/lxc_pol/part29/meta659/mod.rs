//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2187;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2188;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2189;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta659<F: Float>(t1799: F, t3886: F, t22633: F, t22635: F, t3888: F, t80663: F, t80671: F, t1887: F, t80827: F, t26334: F, t26339: F, t81159: F, t22716: F, t7697: F, t16452: F, t26224: F, t26225: F, t80647: F, t80659: F, t80665: F, t80667: F, t80683: F, t90460: F, t90462: F, t90466: F, t90469: F, t90471: F, t90473: F, t90477: F, t90485: F, t1307: F, t1385: F, t26331: F, t26337: F, t26216: F, t26210: F, t6897: F, t794: F, t1377: F, t5187: F, t7692: F, t81186: F, t26338: F, t81228: F, t81326: F, t6888: F, t7691: F, t80707: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t90491, t90493, t90496, t90497, t90498, t90500) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2187::<F>(t1799, t3886, t22633, t22635, t3888, t80663, t80671, t1887, t80827, t26334, t26339, t81159);
        let t90505 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2188::<F>(t90500, t22716, t7697, t16452, t26224, t26225, t80647, t80659, t80665, t80667, t80683, t90460, t90462, t90466, t90469, t90471, t90473, t90477, t90485, t90491, t90493, t90496, t90498);
        let (t90506, t90509, t90512, t90515, t90516) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2189::<F>(t1307, t1385, t22635, t26331, t26337, t26216, t81159, t26210, t6897, t794, t1377, t5187);
        let (t90519, t90521, t90525, t90527) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2190::<F>(t1385, t22633, t22635, t90516, t7692, t81186, t26338, t81228, t81326, t6888, t7691, t80707);
    (t90497, t90505, t90506, t90509, t90512, t90515, t90519, t90521, t90525, t90527)
}
