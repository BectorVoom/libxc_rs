//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 521/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk521<F: Float>(t173: F, t3171: F, t3170: F, t1013: F, t605: F, t3129: F, t3134: F, t3145: F, t3147: F, t3150: F, t3158: F, t3161: F, t3164: F, t3166: F, t3168: F, t3027: F, t3068: F, t3126: F) -> (F, F) {
    let t3172 = t3171 * t173;
    let t3173 = t3170 * t3172;
    let t3175 = t1013 * t605;
    let t3177 = -0.37941869869339964455e-7 * t3129 + 0.67460644627686456803e-7 * t3134 - 0.24620447827856422924e-7 * t3145 + 0.13900948042322754167e-2 * t3147 + 0.10120768229166666667e-4 * t3150 + 0.14759453667534722223e-5 * t3158 + 0.13900948042322754167e-2 * t3161 - 0.14492726735651760868e-5 * t3164 - 0.50603841145833333335e-4 * t3166 + 0.61902840252896149066e-6 * t3168 - 0.34752370105806885418e-4 * t3173 - 0.3243554543208642639e-2 * t3175;
    let t3179 = t3027 + t3068 + t3126 + t3177;
    (t3172, t3179)
}
