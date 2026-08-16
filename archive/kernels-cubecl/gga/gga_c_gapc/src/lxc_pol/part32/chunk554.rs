//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 554/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk554<F: Float>(t169: F, t588: F, t125: F, t481: F, t173: F, t1013: F, t605: F, t3129: F, t3134: F, t3145: F, t3147: F, t3150: F, t3158: F, t3161: F, t3164: F, t3166: F, t3168: F) -> (F, F, F, F) {
    let t3170 = t169 * t588;
    let t3171 = t481 * t125;
    let t3172 = t3171 * t173;
    let t3173 = t3170 * t3172;
    let t3175 = t1013 * t605;
    let t3177 = -F::cast_from(0.37941869869339964455e-7_f64) * t3129 + F::cast_from(0.67460644627686456803e-7_f64) * t3134 - F::cast_from(0.24620447827856422924e-7_f64) * t3145 + F::cast_from(0.13900948042322754167e-2_f64) * t3147 + F::cast_from(0.10120768229166666667e-4_f64) * t3150 + F::cast_from(0.14759453667534722223e-5_f64) * t3158 + F::cast_from(0.13900948042322754167e-2_f64) * t3161 - F::cast_from(0.14492726735651760868e-5_f64) * t3164 - F::cast_from(0.50603841145833333335e-4_f64) * t3166 + F::cast_from(0.61902840252896149066e-6_f64) * t3168 - F::cast_from(0.34752370105806885418e-4_f64) * t3173 - F::cast_from(0.3243554543208642639e-2_f64) * t3175;
    (t3170, t3171, t3172, t3177)
}
