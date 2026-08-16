//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1337/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1337<F: Float>(t3972: F, t3975: F, t9574: F, t1161: F, t353: F, t51084: F, t859: F, t4183: F, t4386: F, t810: F, t1173: F, t9203: F) -> (F, F, F, F) {
    let t54541 = t3972 * t3975 * t9574;
    let t54545 = t859 * t353 * t51084 * t1161;
    let t54550 = t4386 * t353 * t4183 * t810;
    let t54561 = t1173 * t9203;
    (t54541, t54545, t54550, t54561)
}
