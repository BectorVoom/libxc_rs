//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1865;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1866;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1867;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta472(t1510: f64, t17027: f64, t20723: f64, t20724: f64, t20744: f64, t20745: f64, t20751: f64, t9457: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9715: f64, t20760: f64, t20761: f64, t20765: f64, t20766: f64, t20768: f64, t9724: f64, t9780: f64, t9789: f64, t9793: f64, t9797: f64, t9863: f64, t4205: f64, t5597: f64, t185: f64, t20217: f64, t707: f64, t13115: f64, t5499: f64, t20777: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64, t9894: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20806, t20811) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1865(t1510, t17027, t20723, t20724, t20744, t20745, t20751, t9457, t9469, t9476, t9484, t9496, t9715);
        let (t20812, t20815) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1866(t20760, t20761, t20765, t20766, t20768, t9724, t9780, t9789, t9793, t9797, t9863, t4205, t5597);
        let (t20816, t20818, t20820, t20821) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1867(t185, t20217, t707, t13115, t5499, t20777, t20815, t9820, t9824, t9876, t9884, t9887, t9890, t9894);
    (t20806, t20811, t20812, t20815, t20816, t20818, t20820, t20821)
}
