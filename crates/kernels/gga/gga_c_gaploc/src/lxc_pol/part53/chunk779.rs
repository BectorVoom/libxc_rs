//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 779/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk779<F: Float>(t2508: F, t43240: F, t7226: F, t13209: F, t7137: F, t1841: F, t8878: F, t9748: F, t10053: F, t1024: F, t2927: F, t3270: F, t13221: F, t13217: F, t13185: F, t40877: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43243 = 0.46143157380853345701e-1 * t2508 * t7226 * t43240;
    let t43254 = 0.10254034973522965712e-1 * t7137 * t13209;
    let t43257 = 0.25635087433807414279e-2 * t1841 * t8878 * t9748;
    let t43260 = 0.76905262301422242837e-2 * t2508 * t10053 * t1024;
    let t43263 = 0.76905262301422242837e-2 * t2508 * t3270 * t2927;
    let t43265 = 0.10254034973522965712e-1 * t7137 * t13221;
    let t43267 = 0.61524209841137794268e-1 * t7137 * t13217;
    let t43269 = 0.71778244814660759981e-1 * t7137 * t13185;
    let t43274 = 0.85450291446024714264e-3 * t40877;
    (t43243, t43254, t43257, t43260, t43263, t43265, t43267, t43269, t43274)
}
