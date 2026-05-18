//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1056/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1056<F: Float>(t11539: F, t3223: F, t3816: F, t6627: F, t11806: F, t2170: F, t875: F, t2168: F, t6481: F, t2319: F, t3810: F, t3128: F, t8963: F) -> (F, F, F, F, F, F, F) {
    let t11854 = t11539 * t3223;
    let t11857 = t6627 * t3816;
    let t11860 = t2170 * t11806 * t875;
    let t11862 = t2168 * t11860 / F::new(48.0);
    let t11863 = F::new(35.0) / F::new(216.0) * t6481;
    let t11864 = t2319 * t3810;
    let t11867 = t3128 * t8963 / F::new(24.0);
    (t11854, t11857, t11860, t11862, t11863, t11864, t11867)
}
