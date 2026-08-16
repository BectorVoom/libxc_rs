//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 968/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk968(t40: f64, t52: f64, t13107: f64, t1530: f64, t5664: f64, t20217: f64, t20234: f64, t4104: f64, t5398: f64, t634: f64, t767: f64, t4111: f64, t638: f64, t771: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t20777 = 0.51947577317044391276e2_f64 * t13107;
    let t20778 = t5664 * t1530;
    let t20790 = piecewise3(t146, 0.0_f64, 8.0_f64 / 27.0_f64 * t634 * t20234 - 2.0_f64 / 3.0_f64 * t4104 * t5398 + 2.0_f64 / 3.0_f64 * t767 * t20217);
    let t20798 = piecewise3(t150, 0.0_f64, -8.0_f64 / 27.0_f64 * t638 * t20234 - 2.0_f64 / 3.0_f64 * t4111 * t5398 - 2.0_f64 / 3.0_f64 * t771 * t20217);
    (t20777, t20778, t20790, t20798)
}
