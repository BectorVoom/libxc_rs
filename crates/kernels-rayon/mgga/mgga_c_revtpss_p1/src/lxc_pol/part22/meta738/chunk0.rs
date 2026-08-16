//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2799/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2799(t10782: f64, t40731: f64, t159: f64, t33127: f64, t64: f64, t222: f64, t10709: f64, t10760: f64, t9794: f64, t124: f64, t138: f64, t40649: f64, t9645: f64) -> (f64, f64, f64, f64, f64) {
    let t40732 = t40731 * t10782;
    let t40735 = t64 * t33127 * t159;
    let t40737 = 455.0_f64 / 243.0_f64 * t40735 * t222;
    let t40753 = t10760 * t9794 * t10709;
    let t40757 = t138 * t124 * t40649 * t9645;
    (t40732, t40735, t40737, t40753, t40757)
}
