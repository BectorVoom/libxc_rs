//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1865;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1866;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1867;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta472<F: Float>(t1510: F, t17027: F, t20723: F, t20724: F, t20744: F, t20745: F, t20751: F, t9457: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F, t20760: F, t20761: F, t20765: F, t20766: F, t20768: F, t9724: F, t9780: F, t9789: F, t9793: F, t9797: F, t9863: F, t4205: F, t5597: F, t185: F, t20217: F, t707: F, t13115: F, t5499: F, t20777: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F, t9894: F) -> (F, F, F, F, F, F, F, F) {
        let (t20806, t20811) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1865::<F>(t1510, t17027, t20723, t20724, t20744, t20745, t20751, t9457, t9469, t9476, t9484, t9496, t9715);
        let (t20812, t20815) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1866::<F>(t20760, t20761, t20765, t20766, t20768, t9724, t9780, t9789, t9793, t9797, t9863, t4205, t5597);
        let (t20816, t20818, t20820, t20821) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1867::<F>(t185, t20217, t707, t13115, t5499, t20777, t20815, t9820, t9824, t9876, t9884, t9887, t9890, t9894);
    (t20806, t20811, t20812, t20815, t20816, t20818, t20820, t20821)
}
