//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 763/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk763<F: Float>(t12484: F, t173: F, t184: F, t199: F, t12350: F, t5063: F, t5089: F, t11: F, t5002: F, t1691: F, t2678: F, t3354: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12485 = t173 * t12484;
    let t12486 = t12485 * t184;
    let t12488 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t12486 * t199;
    let t12493 = t5063 * t12350;
    let t12494 = t5089 * t12493;
    let t12495 = t11 * t12494;
    let t12497 = t5002 * t12350;
    let t12498 = t1691 * t12497;
    let t12499 = t11 * t12498;
    let t12501 = t2678 * t3354;
    (t12485, t12486, t12488, t12493, t12494, t12495, t12497, t12498, t12499, t12501)
}
