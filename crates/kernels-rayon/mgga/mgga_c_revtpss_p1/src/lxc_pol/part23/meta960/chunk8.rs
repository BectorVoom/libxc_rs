//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3241/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3241(t5: f64, t13269: f64, t13272: f64, t1497: f64, t21663: f64, t21674: f64, t21677: f64, t21682: f64, t4178: f64, t4241: f64, t45931: f64, t45933: f64, t45941: f64, t45944: f64, t45952: f64, t5816: f64, t5872: f64, t60214: f64, t60215: f64, t60216: f64, t60217: f64, t60218: f64, t60221: f64, t60224: f64, t60670: f64, t60673: f64, t644: f64, t85037: f64, t85305: f64, t91: f64) -> f64 {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t85307 = piecewise3(t8, 0.0_f64, (t60214 + t45931 - t45933 - t60215 + t60216 + t45941 - t45944 - t60217 + t60218 + t45952) * t91 - 4.0_f64 * t85037 * t644 - 12.0_f64 * t60670 * t1497 + 60.0_f64 * t60673 * t4178 - 12.0_f64 * t21663 * t4241 + 60.0_f64 * t60221 * t5816 - 360.0_f64 * t60224 * t21674 + 120.0_f64 * t13272 * t21677 - 12.0_f64 * t13269 * t5872 + 60.0_f64 * t13272 * t21682 + t85305);
    t85307
}
