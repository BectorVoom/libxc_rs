//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk964;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk965;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta276(t40: f64, t52: f64, t20217: f64, t20234: f64, t4080: f64, t5398: f64, t73: f64, t9427: f64, t4087: f64, t76: f64, t9438: f64, t157: f64, t182: f64, t16587: f64, zeta_threshold: f64, t4195: f64, t4194: f64, t1530: f64, t17116: f64, t1877: f64, t20723: f64, t20724: f64, t9457: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9715: f64, t9724: f64, t4310: f64, t5527: f64, t1484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20741, t20742, t20744, t20745) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk964(t40, t52, t20217, t20234, t4080, t5398, t73, t9427, t4087, t76, t9438, t157, t182, t16587, zeta_threshold);
        let (t20749, t20751, t20752) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk965(t4195, t5398, t4194, t1530, t17116, t1877, t20723, t20724, t20744, t20745, t9457, t9469, t9476, t9484, t9496, t9715, t9724);
        let (t20753, t20756) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk966(t4310, t5527, t1484);
    (t20741, t20742, t20744, t20745, t20749, t20751, t20752, t20753, t20756)
}
