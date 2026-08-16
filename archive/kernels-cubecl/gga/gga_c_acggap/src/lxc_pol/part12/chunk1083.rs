//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1083/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1083<F: Float>(t1967: F, t8978: F, t33953: F, t5127: F, t13287: F, t31057: F, t4210: F, t13364: F, t31443: F, t3169: F, t2288: F, t3176: F) -> (F, F, F, F, F, F, F) {
    let t35273 = t1967 * t8978;
    let t35284 = t33953 * t5127;
    let t35286 = t31057 * t13287 * t35284;
    let t35288 = t33953 * t4210;
    let t35290 = t31057 * t13364 * t35288;
    let t35294 = t31443 * t13287 * t33953 * t3169;
    let t35296 = t2288 * t3176;
    (t35273, t35284, t35286, t35288, t35290, t35294, t35296)
}
