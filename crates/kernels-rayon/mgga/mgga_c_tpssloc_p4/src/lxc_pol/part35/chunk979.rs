//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 979/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk979(t1510: f64, t17027: f64, t20723: f64, t20724: f64, t20744: f64, t20745: f64, t20751: f64, t9457: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9715: f64) -> (f64, f64) {
    let t20806 = t17027 * t1510;
    let t20811 = t20723 - t9457 + t20724 - t9469 + t20744 + t20745 + t9476 + t9484 - t9496 + t20751 - t9715;
    (t20806, t20811)
}
