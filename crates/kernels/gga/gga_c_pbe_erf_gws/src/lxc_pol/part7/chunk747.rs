//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 747/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk747<F: Float>(t897: F, t2417: F, t938: F, t338: F, t353: F, t2395: F, t810: F, t2370: F, t830: F) -> (F, F, F, F, F, F) {
    let t6125 = t897 * t897;
    let t6126 = F::cast_from(1.0_f64) / t6125;
    let t6127 = t2417 * t938;
    let t6128 = t6126 * t6127;
    let t6130 = t338 * t353 * t6128;
    let t6133 = t2395 * t810;
    let t6135 = t2370 * t830 * t6133;
    (t6125, t6126, t6127, t6128, t6130, t6135)
}
