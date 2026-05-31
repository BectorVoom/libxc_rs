//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1125/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1125<F: Float>(t2190: F, t810: F, t824: F, t2147: F, t337: F, t6325: F, t6326: F, t6705: F, t2120: F, t2112: F, t6345: F, t6319: F, t6535: F) -> (F, F, F, F, F, F) {
    let t20206 = t2190 * t810;
    let t20207 = t824 * t20206;
    let t20215 = t6325 * t2147 * t337 * t6326 * t810 / F::cast_from(4.0_f64);
    let t20219 = t2147 * t337 * t6705 * t810;
    let t20221 = t2120 * t20219 / F::cast_from(12.0_f64);
    let t20222 = t6345 * t2112;
    let t20228 = t6319 * t6535 / F::cast_from(6.0_f64);
    (t20206, t20207, t20215, t20221, t20222, t20228)
}
