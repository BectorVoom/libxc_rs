//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 353/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk353<F: Float>(t1383: F, t169: F, t289: F, t274: F, t39: F, t1216: F, t1319: F, t1322: F) -> (F, F, F) {
    let t1386 = F::new(0.31835665774679373271e-1) * t169 * t289 * t1383;
    let t1388 = F::new(0.3199504064530762818e0) * t39 * t274;
    let t1392 = t1319 * t1216 * t1322;
    (t1386, t1388, t1392)
}
