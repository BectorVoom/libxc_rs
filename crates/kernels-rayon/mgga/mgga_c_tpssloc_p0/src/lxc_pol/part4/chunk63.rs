//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 63/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk63(t123: f64, t126: f64, t129: f64, t136: f64) -> (f64, f64, f64, f64) {
    let t164 = 0.705945e1_f64 * t126 + 0.1549425e1_f64 * t123 + 0.420775e0_f64 * t129 + 0.1562925e0_f64 * t136;
    let t167 = 1.0_f64 + 0.32163958997385070134e2_f64 / t164;
    let t168 = f64::ln(t167);
    let t172 = 1.0_f64 + 0.278125e-1_f64 * t123;
    (t164, t167, t168, t172)
}
