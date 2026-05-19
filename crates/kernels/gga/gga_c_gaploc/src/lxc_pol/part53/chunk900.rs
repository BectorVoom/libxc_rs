//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 900/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk900<F: Float>(t2508: F, t2927: F, t3270: F, t13221: F, t7137: F, t13217: F, t13185: F, t40877: F, t1897: F, t28720: F, t9014: F, t28024: F, t2936: F) -> (F, F, F, F, F, F, F) {
    let t43263 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t3270 * t2927;
    let t43265 = F::cast_from(0.10254034973522965712e-1_f64) * t7137 * t13221;
    let t43267 = F::cast_from(0.61524209841137794268e-1_f64) * t7137 * t13217;
    let t43269 = F::cast_from(0.71778244814660759981e-1_f64) * t7137 * t13185;
    let t43274 = F::cast_from(0.85450291446024714264e-3_f64) * t40877;
    let t43282 = F::cast_from(0.92286314761706691403e-1_f64) * t1897 * t9014 * t28720;
    let t43286 = F::cast_from(0.53833683610995569986e-1_f64) * t2508 * t2936 * t28024;
    (t43263, t43265, t43267, t43269, t43274, t43282, t43286)
}
