//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1312/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1312<F: Float>(t3959: F, t9932: F, t3897: F, t4386: F, t13792: F, t15167: F, t3972: F, t50956: F, t8827: F, t13776: F, t3742: F, t875: F) -> (F, F, F, F) {
    let t56776 = t3959 * t9932;
    let t56782 = t4386 * t3897;
    let t56783 = t13792 * t56782;
    let t56787 = t3972 * t50956 * t8827 * t15167;
    let t56791 = t13776 * t50956 * t3742 * t875;
    (t56776, t56783, t56787, t56791)
}
