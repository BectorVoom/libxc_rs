//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2114/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2114(t15130: f64, t2908: f64, t141: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15163: f64, t15166: f64, t15169: f64, t15170: f64) -> (f64, f64, f64) {
    let t15172 = t2908 * t15130;
    let t15173 = t141 * t15172;
    let t15175 = -0.20128333333333333333e0_f64 * t15137 - 0.33547222222222222222e0_f64 * t15142 + 0.12077e1_f64 * t15147 + 0.60385e0_f64 * t15151 + 0.12077e1_f64 * t15156 - 0.181155e1_f64 * t15160 + 0.16557e0_f64 * t15163 - 0.49671e0_f64 * t15166 - t15169 + 0.36793333333333333334e-1_f64 * t15170 - 0.5519e-1_f64 * t15173;
    (t15172, t15173, t15175)
}
