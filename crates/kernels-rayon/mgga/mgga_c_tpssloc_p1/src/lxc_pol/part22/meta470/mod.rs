//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1860;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta470(t4195: f64, t5398: f64, t4194: f64, t1530: f64, t17116: f64, t1877: f64, t20723: f64, t20724: f64, t20744: f64, t20745: f64, t9457: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9715: f64, t9724: f64, t4310: f64, t5527: f64, t1484: f64) -> (f64, f64, f64, f64, f64) {
        let (t20749, t20751, t20752) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1860(t4195, t5398, t4194, t1530, t17116, t1877, t20723, t20724, t20744, t20745, t9457, t9469, t9476, t9484, t9496, t9715, t9724);
        let (t20753, t20756) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1861(t4310, t5527, t1484);
    (t20749, t20751, t20752, t20753, t20756)
}
