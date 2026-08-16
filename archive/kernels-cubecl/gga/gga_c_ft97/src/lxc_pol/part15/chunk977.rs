//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 977/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk977<F: Float>(t22479: F, t5: F, t21843: F, t2253: F, t21847: F, t21837: F, t2938: F, t21856: F, t21893: F, t21867: F, t668: F, t1268: F, t4635: F) -> (F, F, F, F, F, F, F, F, F) {
    let t82074 = t5 * t22479;
    let t82077 = t2253 * t21843;
    let t82079 = t2253 * t21847;
    let t82082 = t2938 * t21837;
    let t82088 = t2253 * t21856;
    let t82095 = t2253 * t21893;
    let t82097 = t2253 * t21867;
    let t82112 = t21837 * t668;
    let t82182 = t4635 * t1268;
    (t82074, t82077, t82079, t82082, t82088, t82095, t82097, t82112, t82182)
}
