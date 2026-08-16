//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1862;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1863;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta471(t16617: f64, t12943: f64, t16630: f64, t12946: f64, t145: f64, t20741: f64, t185: f64, t4315: f64, t5544: f64, t1484: f64, t16606: f64, t193: f64, t20753: f64, t20756: f64, t2522: f64, t262: f64, t4314: f64, t9780: f64, t9789: f64, t9793: f64, t9797: f64, t9863: f64, t40: f64, t52: f64, t13107: f64, t1530: f64, t5664: f64, t20217: f64, t20234: f64, t4104: f64, t5398: f64, t634: f64, t767: f64, t4111: f64, t638: f64, t771: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20760, t20761, t20765, t20766, t20767, t20768, t20769, t20772) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1862(t16617, t12943, t16630, t12946, t145, t20741, t185, t4315, t5544, t1484, t16606, t193, t20753, t20756, t2522, t262, t4314, t9780, t9789, t9793, t9797, t9863);
        let (t20777, t20778, t20790, t20798) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1863(t40, t52, t13107, t1530, t5664, t20217, t20234, t4104, t5398, t634, t767, t4111, t638, t771, zeta_threshold);
        let t20800 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1864(t20790, t20798);
    (t20760, t20761, t20765, t20766, t20767, t20768, t20769, t20772, t20777, t20778, t20800)
}
