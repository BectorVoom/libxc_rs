//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2042/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2042(t7284: f64, t98087: f64, t7289: f64, t10073: f64, t25937: f64, t7282: f64, t7910: f64, t25899: f64, t97899: f64, t25953: f64, t27899: f64, t25981: f64, t5677: f64, t820: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98089 = 0.14456046980341999104e-1_f64 * t7284 * t98087;
    let t98091 = 0.25702851531048074406e-1_f64 * t7289 * t98087;
    let t98099 = t10073 * t7282 * t25937 * t7910;
    let t98101 = t25899 * t97899;
    let t98104 = t27899 * t25953;
    let t98108 = t820 * t25981 * t844 * t5677;
    (t98089, t98091, t98099, t98101, t98104, t98108)
}
