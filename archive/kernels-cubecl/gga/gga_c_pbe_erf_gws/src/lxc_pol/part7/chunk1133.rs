//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1133/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1133<F: Float>(t6180: F, t6538: F, t6665: F, t810: F, t6184: F, t6188: F, t6183: F, t6410: F, t2134: F, t2156: F, t745: F, t343: F) -> (F, F, F, F, F, F) {
    let t20321 = t6538 * t6180 / F::cast_from(16.0_f64);
    let t20322 = t6665 * t810;
    let t20327 = t6188 * t6184;
    let t20328 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t20327;
    let t20333 = t6183 * t6410;
    let t20334 = t2134 * t20333;
    let t20335 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t20334;
    let t20344 = t745 * t2156;
    let t20345 = t20344 * t343;
    (t20321, t20322, t20328, t20335, t20344, t20345)
}
