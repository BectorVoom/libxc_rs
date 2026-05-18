//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1152/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1152<F: Float>(t11434: F, t3975: F, t13776: F, t2409: F, t8582: F, t3965: F, t2416: F, t345: F) -> (F, F, F, F, F) {
    let t14784 = t3975 * t11434;
    let t14785 = t13776 * t14784;
    let t14787 = t2409 * t8582;
    let t14788 = t3965 * t14787;
    let t14797 = t345 * t2416;
    (t14784, t14785, t14787, t14788, t14797)
}
