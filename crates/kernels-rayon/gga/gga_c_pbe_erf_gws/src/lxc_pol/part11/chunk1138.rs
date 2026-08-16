//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1138/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1138(t12751: f64, t5211: f64, t7106: f64, t32739: f64, t48112: f64, t48113: f64, t48114: f64, t48115: f64, t48117: f64, t48119: f64, t48120: f64, t48122: f64, t48124: f64) -> (f64, f64, f64) {
    let t48127 = 64.0_f64 / 15.0_f64 * t5211 * t7106 * t12751;
    let t48128 = 16.0_f64 / 135.0_f64 * t32739;
    let t48129 = t48112 - t48113 - t48114 - t48115 + t48117 + t48119 + t48120 + t48122 + t48124 + t48127 - t48128;
    (t48127, t48128, t48129)
}
