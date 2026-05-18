//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 624/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk624<F: Float>(t252: F, t5385: F, t245: F, t713: F, t1697: F, t212: F, t22: F, t219: F, t5063: F, t1923: F, t247: F, t24: F) -> (F, F, F, F, F, F, F) {
    let t5387 = F::new(8.0) / F::new(81.0) * t252 * t5385;
    let t5390 = t245 * t713;
    let t5399 = F::new(1.0) / t212 / t1697;
    let t5400 = t22 * t5399;
    let t5401 = t219 * t5063;
    let t5420 = t247 * t1923;
    let t5421 = t24 * t5420;
    (t5387, t5390, t5399, t5400, t5401, t5420, t5421)
}
