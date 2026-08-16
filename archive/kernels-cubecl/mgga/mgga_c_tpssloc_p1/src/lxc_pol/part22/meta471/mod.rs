//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1862;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1863;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta471<F: Float>(t16617: F, t12943: F, t16630: F, t12946: F, t145: F, t20741: F, t185: F, t4315: F, t5544: F, t1484: F, t16606: F, t193: F, t20753: F, t20756: F, t2522: F, t262: F, t4314: F, t9780: F, t9789: F, t9793: F, t9797: F, t9863: F, t40: F, t52: F, t13107: F, t1530: F, t5664: F, t20217: F, t20234: F, t4104: F, t5398: F, t634: F, t767: F, t4111: F, t638: F, t771: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20760, t20761, t20765, t20766, t20767, t20768, t20769, t20772) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1862::<F>(t16617, t12943, t16630, t12946, t145, t20741, t185, t4315, t5544, t1484, t16606, t193, t20753, t20756, t2522, t262, t4314, t9780, t9789, t9793, t9797, t9863);
        let (t20777, t20778, t20790, t20798) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1863::<F>(t40, t52, t13107, t1530, t5664, t20217, t20234, t4104, t5398, t634, t767, t4111, t638, t771, zeta_threshold);
        let t20800 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1864::<F>(t20790, t20798);
    (t20760, t20761, t20765, t20766, t20767, t20768, t20769, t20772, t20777, t20778, t20800)
}
