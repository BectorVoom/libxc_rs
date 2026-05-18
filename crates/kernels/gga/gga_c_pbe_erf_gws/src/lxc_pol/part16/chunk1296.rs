//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1296/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1296<F: Float>(t13953: F, t14781: F, t14001: F, t3062: F, t14772: F, t3972: F, t3975: F, t9574: F, t1173: F, t9203: F, t2409: F, t26668: F, t3965: F) -> (F, F, F, F, F, F) {
    let t54531 = t13953 * t14781;
    let t54535 = t14001 * t3062;
    let t54537 = t14001 * t14772;
    let t54541 = t3972 * t3975 * t9574;
    let t54561 = t1173 * t9203;
    let t54564 = t3965 * t2409 * t26668;
    (t54531, t54535, t54537, t54541, t54561, t54564)
}
