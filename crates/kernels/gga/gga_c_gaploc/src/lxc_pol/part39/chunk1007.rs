//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1007/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1007<F: Float>(t1897: F, t28720: F, t9014: F, t40896: F, t2508: F, t28024: F, t2936: F, t40898: F, t40900: F, t13176: F, t731: F, t22090: F, t28668: F, t8604: F) -> (F, F, F, F, F, F, F) {
    let t43282 = F::new(0.92286314761706691403e-1) * t1897 * t9014 * t28720;
    let t43283 = F::new(0.17090058289204942853e-2) * t40896;
    let t43286 = F::new(0.53833683610995569986e-1) * t2508 * t2936 * t28024;
    let t43288 = F::new(0.85450291446024714264e-3) * t40898;
    let t43289 = F::new(0.85450291446024714264e-3) * t40900;
    let t43290 = t731 * t13176;
    let t43295 = F::new(0.1845726295234133828e0) * t2508 * t22090 * t8604 * t28668;
    (t43282, t43283, t43286, t43288, t43289, t43290, t43295)
}
