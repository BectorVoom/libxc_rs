//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 790/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk790<F: Float>(t2170: F, t2171: F, t6177: F, t2168: F, t2122: F, t337: F, t810: F, t2147: F, t2120: F, t2133: F, t2387: F, t2138: F) -> (F, F, F, F, F, F, F) {
    let t6530 = t2170 * t6177 * t2171;
    let t6532 = t2168 * t6530 / F::new(16.0);
    let t6534 = t337 * t2122 * t810;
    let t6535 = t2147 * t6534;
    let t6537 = t2120 * t6535 / F::new(16.0);
    let t6538 = t2387 * t2133;
    let t6540 = t6538 * t2138 / F::new(32.0);
    (t6530, t6532, t6534, t6535, t6537, t6538, t6540)
}
