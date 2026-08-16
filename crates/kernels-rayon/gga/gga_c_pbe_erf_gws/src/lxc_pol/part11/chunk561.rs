//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 561/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk561(t3116: f64, t3793: f64, t3128: f64, t3180: f64, t3703: f64, t858: f64, t2210: f64, t884: f64, t1112: f64) -> (f64, f64, f64, f64, f64) {
    let t3795 = t3116 * t3793 / 96.0_f64;
    let t3797 = t3128 * t3180 / 24.0_f64;
    let t3798 = t858 * t3703;
    let t3799 = t2210 * t3798;
    let t3801 = t884 * t3799 / 16.0_f64;
    let t3802 = t1112 * t1112;
    (t3795, t3797, t3799, t3801, t3802)
}
