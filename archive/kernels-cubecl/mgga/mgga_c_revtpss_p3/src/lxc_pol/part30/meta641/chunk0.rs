//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2228/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2228<F: Float>(t17608: F, t7617: F, t17217: F, t26880: F, t17376: F, t26843: F, t26848: F, t29010: F, t3704: F, t17720: F, t7624: F, t1252: F, t17199: F, t17204: F, t17232: F, t17589: F, t3606: F, t3613: F, t97125: F) -> F {
    let t104677 = t17608 * t7617;
    let t104680 = t26880 * t17217;
    let t104682 = t17376 * t26843;
    let t104685 = t17376 * t26848;
    let t104689 = F::cast_from(0.57165357490759649296e-3_f64) * t29010 * t3704;
    let t104691 = F::cast_from(0.6351706387862183255e-3_f64) * t7624 * t17720;
    let t104692 = F::cast_from(0.57165357490759649296e-3_f64) * t97125 + F::cast_from(0.57165357490759649296e-3_f64) * t26880 * t17589 - F::cast_from(0.11433071498151929859e-2_f64) * t7624 * t17232 - F::cast_from(0.57165357490759649296e-3_f64) * t7624 * t17199 - F::cast_from(0.17149607247227894789e-2_f64) * t7624 * t17204 + F::cast_from(0.85748036236139473944e-3_f64) * t104677 * t1252 + F::cast_from(0.3811023832717309953e-3_f64) * t104680 + F::cast_from(0.85748036236139473944e-3_f64) * t104682 * t3606 - F::cast_from(0.42874018118069736972e-3_f64) * t104685 * t3613 + t104689 + t104691;
    t104692
}
