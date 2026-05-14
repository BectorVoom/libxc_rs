//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 757/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk757<F: Float>(t44888: F, t7290: F, t43093: F, t43100: F, t1897: F, t35583: F, t954: F, t2508: F, t44712: F, t688: F, t779: F, t1023: F, t44878: F, t35573: F, t3009: F, t32356: F, t7226: F) -> (F, F, F, F, F, F, F, F) {
    let t44889 = t7290 * t44888;
    let t44895 = 0.1281754371690370714e-2 * t43093;
    let t44898 = 0.1281754371690370714e-2 * t43100;
    let t44901 = 0.76905262301422242837e-2 * t1897 * t954 * t35583;
    let t44905 = 0.76905262301422242837e-2 * t2508 * t779 * t44712 * t688;
    let t44906 = t1023 * t44878;
    let t44912 = 0.76905262301422242837e-2 * t1897 * t954 * t35573;
    let t44916 = 0.92286314761706691402e-1 * t2508 * t7226 * t3009 * t32356;
    (t44889, t44895, t44898, t44901, t44905, t44906, t44912, t44916)
}
