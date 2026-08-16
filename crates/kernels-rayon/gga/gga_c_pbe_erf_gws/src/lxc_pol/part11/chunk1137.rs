//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1137/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1137(t32704: f64, t32710: f64, t41418: f64, t41421: f64, t1037: f64, t42011: f64, t10629: f64, t3519: f64, t41447: f64, t3523: f64, t10843: f64, t3527: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48112 = 32.0_f64 / 45.0_f64 * t32704;
    let t48113 = 32.0_f64 / 135.0_f64 * t32710;
    let t48114 = 32.0_f64 / 27.0_f64 * t41418;
    let t48115 = 128.0_f64 / 45.0_f64 * t41421;
    let t48117 = 32.0_f64 / 15.0_f64 * t42011 * t1037;
    let t48119 = 16.0_f64 / 15.0_f64 * t10629 * t3519;
    let t48120 = 64.0_f64 / 45.0_f64 * t41447;
    let t48122 = 16.0_f64 / 9.0_f64 * t10629 * t3523;
    let t48124 = 16.0_f64 / 15.0_f64 * t10843 * t3527;
    (t48112, t48113, t48114, t48115, t48117, t48119, t48120, t48122, t48124)
}
