//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1002/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1002<F: Float>(t32704: F, t32710: F, t41418: F, t41421: F, t1037: F, t42011: F, t10629: F, t3519: F, t41447: F, t3523: F, t10843: F, t3527: F, t12751: F, t5211: F, t7106: F, t32739: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t48112 = 32.0 / 45.0 * t32704;
    let t48113 = 32.0 / 135.0 * t32710;
    let t48114 = 32.0 / 27.0 * t41418;
    let t48115 = 128.0 / 45.0 * t41421;
    let t48117 = 32.0 / 15.0 * t42011 * t1037;
    let t48119 = 16.0 / 15.0 * t10629 * t3519;
    let t48120 = 64.0 / 45.0 * t41447;
    let t48122 = 16.0 / 9.0 * t10629 * t3523;
    let t48124 = 16.0 / 15.0 * t10843 * t3527;
    let t48127 = 64.0 / 15.0 * t5211 * t7106 * t12751;
    let t48128 = 16.0 / 135.0 * t32739;
    (t48112, t48113, t48114, t48115, t48117, t48119, t48120, t48122, t48124, t48127, t48128)
}
