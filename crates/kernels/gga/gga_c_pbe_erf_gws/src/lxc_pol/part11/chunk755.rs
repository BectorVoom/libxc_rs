//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 755/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk755<F: Float>(t13086: F, t382: F, t804: F, t3780: F, t829: F, t830: F, t831: F, t1076: F, t1109: F, t1118: F, t353: F, t4386: F) -> (F, F, F, F, F, F) {
    let t13087 = t382 * t13086;
    let t13088 = t804 * t13087;
    let t13096 = t829 * t830 * t831 * t3780;
    let t13105 = t829 * t830 * t831 * t1076;
    let t13110 = t1118 * t1109;
    let t13111 = t353 * t13110;
    let t13112 = t4386 * t13111;
    (t13087, t13088, t13096, t13105, t13110, t13112)
}
