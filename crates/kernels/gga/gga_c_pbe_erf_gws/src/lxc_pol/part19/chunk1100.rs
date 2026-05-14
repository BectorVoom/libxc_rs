//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1100/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1100<F: Float>(t3973: F, t54498: F, t13953: F, t14787: F, t14781: F, t14001: F, t3062: F, t14772: F, t14466: F, t14765: F, t3074: F, t4395: F, t1161: F, t874: F, t3102: F, t859: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54499 = t3973 * t54498;
    let t54504 = t13953 * t14787;
    let t54531 = t13953 * t14781;
    let t54535 = t14001 * t3062;
    let t54537 = t14001 * t14772;
    let t54566 = t14001 * t14466;
    let t54580 = t3074 * t4395 * t14765;
    let t54590 = t1161 * t874;
    let t54595 = t859 * t3102;
    (t54499, t54504, t54531, t54535, t54537, t54566, t54580, t54590, t54595)
}
