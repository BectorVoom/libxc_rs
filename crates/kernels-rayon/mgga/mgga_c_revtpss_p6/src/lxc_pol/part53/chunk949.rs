//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 949/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk949(t531: f64, t7933: f64, t7238: f64, t2014: f64, t1450: f64, t5591: f64, t7237: f64, t13648: f64, t2034: f64, t25190: f64, t7900: f64, t5542: f64, t7312: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28172 = t531 * t7933;
    let t28173 = t28172 * t7238;
    let t28175 = 3.0_f64 * t2014 * t28173;
    let t28176 = t1450 * t5591;
    let t28177 = t7237 * t28176;
    let t28179 = 3.0_f64 * t2014 * t28177;
    let t28182 = t2034 * t13648;
    let t28183 = t2014 * t28182;
    let t28184 = t25190 * t7900;
    let t28186 = 3.0_f64 * t2014 * t28184;
    let t28187 = t7312 * t5542;
    (t28173, t28175, t28176, t28177, t28179, t28182, t28183, t28184, t28186, t28187)
}
