//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 767/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk767<F: Float>(t2206: F, t2216: F, t346: F, t4408: F, t2100: F, t5: F, t337: F, t2121: F, t2271: F, t822: F, t2273: F, t2319: F, t2332: F, t899: F, t900: F, t907: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6691 = t2206 * t2216;
    let t6701 = t4408 * t346;
    let t6705 = t5 * t2100;
    let t6706 = t337 * t6705;
    let t6707 = t2121 * t6706;
    let t6710 = t2271 * t346;
    let t6711 = t822 * t6710;
    let t6714 = t2319 * t2273;
    let t6717 = t899 * t900 * t2332;
    let t6718 = t6717 * t907;
    (t6691, t6701, t6706, t6707, t6710, t6711, t6714, t6717, t6718)
}
