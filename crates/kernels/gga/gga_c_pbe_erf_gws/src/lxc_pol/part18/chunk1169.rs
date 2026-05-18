//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1169/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1169<F: Float>(t3298: F, t3975: F, t3972: F, t4161: F, t840: F, t2409: F, t9707: F, t3965: F, t11434: F, t13776: F, t8582: F, t1192: F, t6126: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14776 = t3975 * t3298;
    let t14777 = t3972 * t14776;
    let t14779 = t840 * t4161;
    let t14781 = t2409 * t9707;
    let t14782 = t3965 * t14781;
    let t14784 = t3975 * t11434;
    let t14785 = t13776 * t14784;
    let t14787 = t2409 * t8582;
    let t14788 = t3965 * t14787;
    let t14791 = t6126 * t1192;
    (t14776, t14777, t14779, t14781, t14782, t14784, t14785, t14787, t14788, t14791)
}
