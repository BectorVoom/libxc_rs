//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1194/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1194<F: Float>(t6241: F, t810: F, t20296: F, t2170: F, t8903: F, t3138: F, t6287: F, t2168: F, t6177: F, t6523: F, t6524: F, t6238: F, t837: F, t863: F) -> (F, F, F, F, F) {
    let t21227 = t6241 * t810;
    let t21231 = t8903 * t2170 * t20296 * t21227 / F::new(2.0);
    let t21239 = t3138 * t2170 * t20296 * t6287 / F::new(2.0);
    let t21243 = F::new(3.0) / F::new(8.0) * t2168 * t6523 * t6177 * t6524;
    let t21245 = t863 * t6238 * t837;
    (t21227, t21231, t21239, t21243, t21245)
}
