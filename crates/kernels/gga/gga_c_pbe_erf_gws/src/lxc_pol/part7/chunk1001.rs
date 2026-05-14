//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1001/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1001<F: Float>(t6563: F, t6711: F, t2074: F, t816: F, t2271: F, t6277: F, t2190: F, t810: F, t824: F, t2147: F, t337: F, t6325: F, t6326: F, t6705: F, t2120: F, t2112: F, t6345: F) -> (F, F, F, F, F, F, F, F) {
    let t20196 = 3.0 / 8.0 * t6711 * t6563;
    let t20197 = t816 * t2074;
    let t20202 = t2271 * t6277;
    let t20206 = t2190 * t810;
    let t20207 = t824 * t20206;
    let t20215 = t6325 * t2147 * t337 * t6326 * t810 / 4.0;
    let t20219 = t2147 * t337 * t6705 * t810;
    let t20221 = t2120 * t20219 / 12.0;
    let t20222 = t6345 * t2112;
    (t20196, t20197, t20202, t20206, t20207, t20215, t20221, t20222)
}
