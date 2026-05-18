//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1062/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1062<F: Float>(t147: F, t159: F, t285: F, t4259: F, t169: F, t301: F, t745: F, t922: F, t5631: F, t755: F, t759: F, t1452: F, t366: F) -> (F, F, F, F, F) {
    let t19174 = F::new(0.10943113336969376162e-5) * t4259 * t147 * t159 * t285;
    let t19177 = t169 * t922 * t745 * t301;
    let t19179 = t5631 * t755;
    let t19182 = F::new(0.78054266140918933351e0) * t5631 * t759;
    let t19185 = t169 * t366 * t1452 * t301;
    (t19174, t19177, t19179, t19182, t19185)
}
