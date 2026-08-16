//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 808/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk808(t10657: f64, t871: f64, t2919: f64, t3113: f64, t1843: f64, t32261: f64, t7064: f64, t2558: f64, t33360: f64, t9647: f64, t13194: f64, t1841: f64) -> (f64, f64, f64, f64, f64) {
    let t43072 = t10657 * t871;
    let t43073 = t2919 * t3113;
    let t43090 = t7064 * t1843 * t32261;
    let t43093 = t9647 * t33360 * t2558;
    let t43095 = t1841 * t13194;
    (t43072, t43073, t43090, t43093, t43095)
}
