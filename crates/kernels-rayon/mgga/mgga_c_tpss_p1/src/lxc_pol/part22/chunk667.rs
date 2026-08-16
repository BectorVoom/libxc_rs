//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 667/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk667(t1139: f64, t3144: f64, t1136: f64, t1149: f64, t3111: f64, t3113: f64, t3120: f64, t473: f64, t1151: f64, t475: f64, t1153: f64, t198: f64, t2856: f64, t2859: f64, t2866: f64, t2908: f64, t2916: f64, t3006: f64, t3008: f64, t3011: f64, t3015: f64, t3019: f64, t3023: f64, t330: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3145 = t1139 * t3144;
    let t3147 = 2.0_f64 * t1136 * t3120 - t1136 * t3145 - 2.0_f64 * t1149 * t3113 + t3111 * t473;
    let t3151 = t1151 * t1151;
    let t3153 = t475 * t475;
    let t3154 = 1.0_f64 / t3153;
    let t3157 = t1153 * t198 * t3147 * t330 - t198 * t3151 * t3154 * t330 - t2856 + t2859 - t2866 + t2908 + t2916 + t3006 + t3008 - t3011 + t3015 - t3019 - t3023;
    (t3145, t3147, t3151, t3153, t3154, t3157)
}
