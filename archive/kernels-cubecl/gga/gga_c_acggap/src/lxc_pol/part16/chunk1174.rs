//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1174/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1174<F: Float>(t6148: F, t7822: F, t30219: F, t9720: F, t4680: F, t7575: F, t9719: F, t31603: F, t31605: F, t35814: F, t35817: F, t35828: F, t37733: F, t37736: F, t40166: F, t40168: F, t40170: F, t40172: F, t40174: F, t40179: F, t40181: F, t40183: F) -> F {
    let t40185 = t7822 * t6148;
    let t40187 = t30219 * t9720;
    let t40190 = t7575 * t4680 * t9719;
    let t40192 = -F::cast_from(0.17149607247227894789e-2_f64) * t40166 - F::cast_from(0.17149607247227894789e-2_f64) * t40168 - F::cast_from(0.40015750243531754508e-1_f64) * t40170 - F::cast_from(0.85748036236139473944e-3_f64) * t40172 - F::cast_from(0.85748036236139473944e-3_f64) * t40174 + F::cast_from(13.0_f64) / F::cast_from(288.0_f64) * t31603 + F::cast_from(0.19055119163586549765e-2_f64) * t31605 + F::cast_from(0.80031500487063509015e-2_f64) * t35814 + t35817 + t37733 + t35828 - t37736 - F::cast_from(0.85748036236139473944e-3_f64) * t40179 + F::cast_from(0.85748036236139473944e-3_f64) * t40181 - F::cast_from(0.85748036236139473944e-3_f64) * t40183 - F::cast_from(0.42874018118069736972e-3_f64) * t40185 + F::cast_from(0.31448092289604152068e-2_f64) * t40187 + F::cast_from(0.31448092289604152068e-2_f64) * t40190;
    t40192
}
