//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1253/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1253<F: Float>(t31605: F, t35812: F, t35818: F, t35829: F, t37731: F, t37732: F, t37735: F, t40166: F, t40168: F, t40170: F, t40172: F, t40174: F, t40179: F, t40181: F, t40183: F, t40185: F, t40187: F, t40190: F) -> F {
    let t41988 = -F::cast_from(0.34299214494455789578e-2_f64) * t40166 - F::cast_from(0.34299214494455789578e-2_f64) * t40168 - F::cast_from(0.80031500487063509015e-1_f64) * t40170 - F::cast_from(0.17149607247227894789e-2_f64) * t40172 - F::cast_from(0.17149607247227894789e-2_f64) * t40174 + t35812 + F::cast_from(0.38110238327173099532e-2_f64) * t31605 + t37731 + t37732 + F::cast_from(0.57165357490759649296e-3_f64) * t35818 + t37735 - F::cast_from(0.32012600194825403606e-1_f64) * t35829 - F::cast_from(0.17149607247227894789e-2_f64) * t40179 + F::cast_from(0.17149607247227894789e-2_f64) * t40181 - F::cast_from(0.17149607247227894789e-2_f64) * t40183 - F::cast_from(0.85748036236139473944e-3_f64) * t40185 + F::cast_from(0.62896184579208304137e-2_f64) * t40187 + F::cast_from(0.62896184579208304137e-2_f64) * t40190;
    t41988
}
