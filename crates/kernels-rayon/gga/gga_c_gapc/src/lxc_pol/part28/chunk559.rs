//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 559/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk559(t169: f64, t588: f64, t125: f64, t481: f64, t173: f64, t1013: f64, t605: f64, t3129: f64, t3134: f64, t3145: f64, t3147: f64, t3150: f64, t3158: f64, t3161: f64, t3164: f64, t3166: f64, t3168: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3170 = t169 * t588;
    let t3171 = t481 * t125;
    let t3172 = t3171 * t173;
    let t3173 = t3170 * t3172;
    let t3175 = t1013 * t605;
    let t3177 = -0.37941869869339964455e-7_f64 * t3129 + 0.67460644627686456803e-7_f64 * t3134 - 0.24620447827856422924e-7_f64 * t3145 + 0.13900948042322754167e-2_f64 * t3147 + 0.10120768229166666667e-4_f64 * t3150 + 0.14759453667534722223e-5_f64 * t3158 + 0.13900948042322754167e-2_f64 * t3161 - 0.14492726735651760868e-5_f64 * t3164 - 0.50603841145833333335e-4_f64 * t3166 + 0.61902840252896149066e-6_f64 * t3168 - 0.34752370105806885418e-4_f64 * t3173 - 0.3243554543208642639e-2_f64 * t3175;
    (t3170, t3171, t3172, t3173, t3175, t3177)
}
