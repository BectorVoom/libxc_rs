//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 894/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk894(t109: f64, t659: f64, t8138: f64, t64: f64, t8127: f64, t8128: f64, t8130: f64, t8134: f64, t8137: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t8139 = t8138 * t659;
    let t8143 = piecewise3(t110, 0.0_f64, t8127 + t8128 * t8130 / 4.0_f64 + 5.0_f64 / 24.0_f64 * t64 * t8134 - 5.0_f64 / 24.0_f64 * t8137 * t8139);
    (t8139, t8143)
}
