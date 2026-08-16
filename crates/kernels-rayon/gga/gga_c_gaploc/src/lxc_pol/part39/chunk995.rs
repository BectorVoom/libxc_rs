//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 995/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk995(t1843: f64, t32261: f64, t7064: f64, t2558: f64, t33360: f64, t9647: f64, t13194: f64, t1841: f64, t13200: f64, t13182: f64, t29439: f64, t13179: f64, t7137: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43090 = t7064 * t1843 * t32261;
    let t43093 = t9647 * t33360 * t2558;
    let t43094 = 0.64087718584518535698e-3_f64 * t43093;
    let t43095 = t1841 * t13194;
    let t43096 = 0.17090058289204942852e-2_f64 * t43095;
    let t43098 = t1841 * t13200;
    let t43099 = 0.2563508743380741428e-2_f64 * t43098;
    let t43100 = t29439 * t13182;
    let t43101 = 0.64087718584518535698e-3_f64 * t43100;
    let t43102 = t7137 * t13179;
    (t43090, t43094, t43096, t43099, t43101, t43102)
}
