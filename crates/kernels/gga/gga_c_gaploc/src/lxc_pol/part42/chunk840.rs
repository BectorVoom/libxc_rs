//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 840/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk840<F: Float>(t43093: F, t43100: F, t1897: F, t35583: F, t954: F, t2508: F, t44712: F, t688: F, t779: F, t35573: F, t3009: F, t32356: F, t7226: F) -> (F, F, F, F, F, F) {
    let t44895 = F::new(0.1281754371690370714e-2) * t43093;
    let t44898 = F::new(0.1281754371690370714e-2) * t43100;
    let t44901 = F::new(0.76905262301422242837e-2) * t1897 * t954 * t35583;
    let t44905 = F::new(0.76905262301422242837e-2) * t2508 * t779 * t44712 * t688;
    let t44912 = F::new(0.76905262301422242837e-2) * t1897 * t954 * t35573;
    let t44916 = F::new(0.92286314761706691402e-1) * t2508 * t7226 * t3009 * t32356;
    (t44895, t44898, t44901, t44905, t44912, t44916)
}
