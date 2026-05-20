//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 706/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk706<F: Float>(t4457: F, t800: F, t2749: F, t4365: F, t2747: F, t2488: F, t2653: F, t2666: F, t2678: F, t2691: F, t2695: F, t2702: F, t2716: F, t2730: F, t2739: F, t2745: F, t4442: F, t4447: F, t4452: F, t4455: F, t799: F) -> (F, F, F) {
    let t4458 = t800 * t4457;
    let t4461 = t4365 * t2749;
    let t4462 = t2747 * t4461;
    let t4468 = t2716 - F::cast_from(0.12705000702321332056e-4_f64) * t2488 + t2730 * t4442 / F::new(16.0) + t2691 + F::cast_from(0.28582678745379824648e-4_f64) * t2695 + t2702 - t2739 - F::cast_from(0.21437009059034868486e-3_f64) * t2745 * t4447 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t4452 + F::new(7.0) / F::new(144.0) * t4455 - t799 * t4458 / F::new(48.0) + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t4462 + F::cast_from(0.40015750243531754508e-2_f64) * t2653 + F::cast_from(0.71456696863449561619e-5_f64) * t2666 - F::cast_from(0.50820002809285328224e-4_f64) * t2678;
    (t4458, t4462, t4468)
}
