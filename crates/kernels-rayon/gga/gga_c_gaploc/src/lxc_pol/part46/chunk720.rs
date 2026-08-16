//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 720/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk720(t13176: f64, t943: f64, t10789: f64, t948: f64, t2508: f64, t10924: f64, t2558: f64, t9647: f64, t1029: f64, t3276: f64, t3433: f64, t954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13177 = t943 * t13176;
    let t13179 = t10789 * t948;
    let t13180 = t2508 * t13179;
    let t13182 = t10924 * t2558;
    let t13183 = t9647 * t13182;
    let t13184 = 0.64087718584518535698e-3_f64 * t13183;
    let t13185 = t3276 * t1029;
    let t13187 = 0.53833683610995569986e-1_f64 * t2508 * t13185;
    let t13188 = t954 * t3433;
    (t13177, t13179, t13180, t13182, t13184, t13185, t13187, t13188)
}
