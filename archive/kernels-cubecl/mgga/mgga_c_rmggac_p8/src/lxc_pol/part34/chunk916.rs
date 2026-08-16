//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 916/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk916<F: Float>(t15078: F, t6444: F, t5259: F, t76052: F, t14302: F, t75299: F, t15137: F, t69444: F, t13823: F, t8465: F, t935: F, t26287: F, t76317: F) -> (F, F, F, F, F, F) {
    let t76440 = t6444 * t15078;
    let t76476 = F::cast_from(0.5987120850931904282e-1_f64) * t5259 * t76052;
    let t76477 = t14302 * t75299;
    let t76479 = t69444 * t15137;
    let t76492 = t13823 * t8465 * t935;
    let t76495 = F::cast_from(0.17961362552795712846e0_f64) * t26287 * t76317;
    (t76440, t76476, t76477, t76479, t76492, t76495)
}
