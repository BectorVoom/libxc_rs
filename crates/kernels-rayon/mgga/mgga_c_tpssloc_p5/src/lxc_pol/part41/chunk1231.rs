//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1231/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1231(t28: f64, t12072: f64, t6312: f64, t3672: f64, t5966: f64, t1081: f64, t18196: f64, t2219: f64, t5142: f64, t517: f64, t157: f64, t19558: f64, t184: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t19559 = t12072 * t6312;
    let t19564 = t3672 * t5966;
    let t19570 = piecewise3(t29, 0.0_f64, -8.0_f64 / 27.0_f64 * t19559 * t1081 - 16.0_f64 / 9.0_f64 * t5142 * t2219 + 4.0_f64 / 9.0_f64 * t19564 * t1081 + 4.0_f64 / 3.0_f64 * t517 * t18196);
    let t19572 = (t19558 + t19570) * t157;
    let t19573 = t19572 * t184;
    (t19572, t19573)
}
