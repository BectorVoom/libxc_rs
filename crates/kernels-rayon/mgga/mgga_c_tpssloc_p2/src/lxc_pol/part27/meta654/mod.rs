//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2282;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2283;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2284;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta654(t90472: f64, t1385: f64, t1992: f64, t22635: f64, t3886: f64, t5353: f64, t3888: f64, t55118: f64, t1799: f64, t22633: f64, t80663: f64, t80671: f64, t1887: f64, t80827: f64, t26334: f64, t26339: f64, t81159: f64, t22716: f64, t7697: f64, t16452: f64, t26224: f64, t26225: f64, t80647: f64, t80659: f64, t80665: f64, t80667: f64, t80683: f64, t90460: f64, t90462: f64, t90466: f64, t90469: f64, t90471: f64, t1307: f64, t26331: f64, t26337: f64, t26216: f64, t26210: f64, t6897: f64, t794: f64, t1377: f64, t5187: f64, t7692: f64, t81186: f64, t26338: f64, t81228: f64, t81326: f64, t6888: f64, t7691: f64, t80707: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90473, t90477, t90485, t90491, t90493, t90496) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2282(t90472, t1385, t1992, t22635, t3886, t5353, t3888, t55118, t1799, t22633, t80663, t80671);
        let (t90497, t90505) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2283(t1887, t80827, t26334, t26339, t81159, t22716, t7697, t16452, t26224, t26225, t80647, t80659, t80665, t80667, t80683, t90460, t90462, t90466, t90469, t90471, t90473, t90477, t90485, t90491, t90493, t90496);
        let (t90506, t90509, t90512, t90515, t90516) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2284(t1307, t1385, t22635, t26331, t26337, t26216, t81159, t26210, t6897, t794, t1377, t5187);
        let (t90519, t90521, t90525, t90527) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2285(t1385, t22633, t22635, t90516, t7692, t81186, t26338, t81228, t81326, t6888, t7691, t80707);
    (t90497, t90505, t90506, t90509, t90512, t90515, t90519, t90521, t90525, t90527)
}
