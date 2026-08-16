//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 79/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk79(t158: f64, t190: f64, t157: f64, t162: f64, t187: f64) -> (f64, f64, f64, f64) {
    let t191 = t158 * t190;
    let t192 = t157 * t162;
    let t194 = 0.19751673498613801407e-1_f64 * t192 * t187;
    let t195 = f64::ln(2.0_f64);
    let t196 = 1.0_f64 - t195;
    (t191, t192, t194, t196)
}
