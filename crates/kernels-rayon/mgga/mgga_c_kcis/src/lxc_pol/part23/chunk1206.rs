//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1206/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1206(t17449: f64, t491: f64, t7949: f64, t3734: f64, t5929: f64, t11825: f64, t27543: f64, t17464: f64, t17402: f64, t7952: f64, t8196: f64, t94754: f64) -> (f64, f64, f64, f64, f64) {
    let t97701 = t17449 * t491;
    let t97702 = t97701 * t7949;
    let t97704 = t3734 * t5929;
    let t97706 = t11825 * t27543;
    let t97707 = t97706 * t17464;
    let t97709 = t7952 * t17402;
    let t97711 = t94754 * t8196;
    (t97702, t97704, t97707, t97709, t97711)
}
