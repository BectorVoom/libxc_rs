//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1088/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1088(t11: f64, t1758: f64, t47442: f64, t47446: f64, t47348: f64, t571: f64, t47438: f64, t47450: f64, t4949: f64, t1856: f64, t24074: f64, t24088: f64, t25: f64, t31777: f64, t47405: f64, t47410: f64, t47418: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47470 = t11 * t1758 * t47442;
    let t47473 = t11 * t1758 * t47446;
    let t47476 = t11 * t571 * t47348;
    let t47479 = t11 * t571 * t47438;
    let t47482 = t11 * t4949 * t47450;
    let t47487 = 0.39999999999999999999e-1_f64 * t25 * t606 * t47418 - 0.66666666666666666666e-2_f64 * t25 * t1856 * t47410 - 0.79999999999999999998e-1_f64 * t25 * t1856 * t47405 + 0.86380000000000000002e0_f64 * t47470 - 0.9597777777777777778e-1_f64 * t47473 - 0.12957e1_f64 * t47476 + 0.28793333333333333333e0_f64 * t47479 - 0.23994444444444444446e0_f64 * t47482 + 0.79012345679012345678e-1_f64 * t24074 + 0.14929876543209876543e0_f64 * t24088 + 0.88888888888888888889e-1_f64 * t31777;
    (t47470, t47473, t47476, t47479, t47482, t47487)
}
