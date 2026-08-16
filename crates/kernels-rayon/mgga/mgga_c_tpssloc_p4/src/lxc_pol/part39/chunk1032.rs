//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1032/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1032(t12808: f64, t656: f64, t12747: f64, t12750: f64, t12752: f64, t12754: f64, t12758: f64, t12761: f64, t64: f64, t9358: f64, t9359: f64, t9361: f64, t9363: f64) -> f64 {
    let t12809 = t656 * t12808;
    let t12812 = -t9358 - 22.0_f64 / 9.0_f64 * t9359 - 2.0_f64 / 3.0_f64 * t9361 + t9363 / 3.0_f64 - 11.0_f64 / 9.0_f64 * t12747 - t12750 + t12752 - 3.0_f64 / 4.0_f64 * t64 * t12754 + t64 * t12758 / 2.0_f64 + t64 * t12761 / 4.0_f64 - t64 * t12809 / 8.0_f64;
    t12812
}
