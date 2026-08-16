//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1351/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1351(t231: f64, t40250: f64, t10639: f64, t10657: f64, t2754: f64, t2815: f64, t39707: f64, t39712: f64, t39714: f64, t39719: f64, t39723: f64, t39724: f64, t39726: f64, t39731: f64, t4514: f64, t820: f64, t837: f64, t879: f64) -> (f64, f64) {
    let t40251 = t40250 * t231;
    let t40255 = 0.65854491829355115985e-1_f64 * t39707 - 0.13170898365871023197e0_f64 * t39712 - 0.26341796731742046395e1_f64 * t4514 * t39714 * t837 + 0.78548797528808629095e-3_f64 * t39719 - t39723 + 0.1040793657534163522e-1_f64 * t39724 - 0.43902994552903410657e-1_f64 * t39726 - 0.26341796731742046395e1_f64 * t820 * t2815 * t10639 + 0.15611904863012452831e0_f64 * t39731 - 0.39512695097613069592e1_f64 * t820 * t10657 * t2754 - 0.65854491829355115987e0_f64 * t820 * t879 * t40251;
    (t40251, t40255)
}
