//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1253/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1253(t31605: f64, t35812: f64, t35818: f64, t35829: f64, t37731: f64, t37732: f64, t37735: f64, t40166: f64, t40168: f64, t40170: f64, t40172: f64, t40174: f64, t40179: f64, t40181: f64, t40183: f64, t40185: f64, t40187: f64, t40190: f64) -> f64 {
    let t41988 = -0.34299214494455789578e-2_f64 * t40166 - 0.34299214494455789578e-2_f64 * t40168 - 0.80031500487063509015e-1_f64 * t40170 - 0.17149607247227894789e-2_f64 * t40172 - 0.17149607247227894789e-2_f64 * t40174 + t35812 + 0.38110238327173099532e-2_f64 * t31605 + t37731 + t37732 + 0.57165357490759649296e-3_f64 * t35818 + t37735 - 0.32012600194825403606e-1_f64 * t35829 - 0.17149607247227894789e-2_f64 * t40179 + 0.17149607247227894789e-2_f64 * t40181 - 0.17149607247227894789e-2_f64 * t40183 - 0.85748036236139473944e-3_f64 * t40185 + 0.62896184579208304137e-2_f64 * t40187 + 0.62896184579208304137e-2_f64 * t40190;
    t41988
}
