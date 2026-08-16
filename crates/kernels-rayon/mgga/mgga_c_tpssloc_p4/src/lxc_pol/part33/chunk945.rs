//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 945/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk945(t4195: f64, t5398: f64, t4194: f64, t1530: f64, t17116: f64, t1877: f64, t20723: f64, t20724: f64, t20744: f64, t20745: f64, t9457: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9715: f64, t9724: f64) -> (f64, f64) {
    let t20749 = t4195 * t5398;
    let t20751 = 36.0_f64 * t4194 * t20749;
    let t20752 = -3.0_f64 * t1530 * t17116 * t1877 + t20723 + t20724 + t20744 + t20745 + t20751 - t9457 - t9469 + t9476 + t9484 - t9496 - t9715 + t9724;
    (t20751, t20752)
}
