//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 618/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk618<F: Float>(t191: F, t784: F, t190: F, t212: F, t205: F, t626: F, t1641: F, t261: F, t174: F, t838: F, t1639: F, t56: F) -> (F, F, F, F, F, F, F) {
    let t5044 = t784 * t191;
    let t5047 = F::cast_from(0.29629629629629629629e-1_f64) * t190 * t5044 * t212;
    let t5060 = F::new(1.0) / t205 / t626;
    let t5061 = t191 * t5060;
    let t5063 = F::new(1.0) / t1641 / t261;
    let t5081 = t174 * t838 * t205;
    let t5082 = F::cast_from(0.11197407407407407407e0_f64) * t5081;
    let t5089 = t56 * t1639;
    (t5044, t5047, t5061, t5063, t5081, t5082, t5089)
}
