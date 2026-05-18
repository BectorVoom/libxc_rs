//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 951/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk951<F: Float>(t10053: F, t1024: F, t2508: F, t2927: F, t3270: F, t13221: F, t7137: F, t13217: F, t13185: F, t42920: F, t723: F, t40877: F) -> (F, F, F, F, F, F, F) {
    let t43260 = F::new(0.76905262301422242837e-2) * t2508 * t10053 * t1024;
    let t43263 = F::new(0.76905262301422242837e-2) * t2508 * t3270 * t2927;
    let t43265 = F::new(0.10254034973522965712e-1) * t7137 * t13221;
    let t43267 = F::new(0.61524209841137794268e-1) * t7137 * t13217;
    let t43269 = F::new(0.71778244814660759981e-1) * t7137 * t13185;
    let t43270 = t42920 * t723;
    let t43274 = F::new(0.85450291446024714264e-3) * t40877;
    (t43260, t43263, t43265, t43267, t43269, t43270, t43274)
}
