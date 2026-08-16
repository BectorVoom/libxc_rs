//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 808/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk808(t28648: f64, t5539: f64, t7064: f64, t28652: f64, t9647: f64, t1843: f64, t28302: f64, t28703: f64, t883: f64, t2537: f64, t2558: f64, t28431: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40828 = t7064 * t5539 * t28648;
    let t40833 = t9647 * t5539 * t28652;
    let t40836 = t7064 * t1843 * t28302;
    let t40848 = t883 * t28703;
    let t40850 = t9647 * t2537 * t40848;
    let t40853 = t9647 * t28431 * t2558;
    (t40828, t40833, t40836, t40848, t40850, t40853)
}
