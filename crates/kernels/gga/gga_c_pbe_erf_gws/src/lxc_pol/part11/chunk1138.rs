//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1138/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1138<F: Float>(t12751: F, t5211: F, t7106: F, t32739: F, t48112: F, t48113: F, t48114: F, t48115: F, t48117: F, t48119: F, t48120: F, t48122: F, t48124: F) -> (F, F, F) {
    let t48127 = F::new(64.0) / F::new(15.0) * t5211 * t7106 * t12751;
    let t48128 = F::new(16.0) / F::new(135.0) * t32739;
    let t48129 = t48112 - t48113 - t48114 - t48115 + t48117 + t48119 + t48120 + t48122 + t48124 + t48127 - t48128;
    (t48127, t48128, t48129)
}
