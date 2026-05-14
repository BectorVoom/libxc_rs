//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1038/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1038<F: Float>(t31603: F, t31605: F, t35814: F, t35817: F, t35828: F, t37733: F, t37736: F, t40166: F, t40168: F, t40170: F, t40172: F, t40174: F, t40179: F, t40181: F, t40183: F, t40185: F, t40187: F, t40190: F) -> (F,) {
    let t40192 = -0.17149607247227894789e-2 * t40166 - 0.17149607247227894789e-2 * t40168 - 0.40015750243531754508e-1 * t40170 - 0.85748036236139473944e-3 * t40172 - 0.85748036236139473944e-3 * t40174 + 13.0 / 288.0 * t31603 + 0.19055119163586549765e-2 * t31605 + 0.80031500487063509015e-2 * t35814 + t35817 + t37733 + t35828 - t37736 - 0.85748036236139473944e-3 * t40179 + 0.85748036236139473944e-3 * t40181 - 0.85748036236139473944e-3 * t40183 - 0.42874018118069736972e-3 * t40185 + 0.31448092289604152068e-2 * t40187 + 0.31448092289604152068e-2 * t40190;
    (t40192,)
}
