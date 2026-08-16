//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 866/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk866(t28176: f64, t7237: f64, t2014: f64, t13648: f64, t2034: f64, t25190: f64, t7900: f64, t5542: f64, t7312: f64, t7315: f64, t7934: f64, t7235: f64, t7901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28177 = t7237 * t28176;
    let t28179 = 3.0_f64 * t2014 * t28177;
    let t28182 = t2034 * t13648;
    let t28183 = t2014 * t28182;
    let t28184 = t25190 * t7900;
    let t28186 = 3.0_f64 * t2014 * t28184;
    let t28187 = t7312 * t5542;
    let t28188 = t2014 * t28187;
    let t28189 = t7934 * t7315;
    let t28190 = t2014 * t28189;
    let t28192 = 3.0_f64 * t7235 * t7901;
    (t28177, t28179, t28182, t28183, t28184, t28186, t28187, t28188, t28189, t28190, t28192)
}
