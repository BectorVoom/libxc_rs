//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 904/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk904(t17068: f64, t1627: f64, t4998: f64, t1631: f64, t5467: f64, t1893: f64, t5207: f64, t579: f64, t5563: f64, t5179: f64, t1778: f64, t1783: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17069 = 32.0_f64 / 135.0_f64 * t17068;
    let t17070 = t1627 * t4998;
    let t17071 = 64.0_f64 / 45.0_f64 * t17070;
    let t17072 = t5467 * t1631;
    let t17073 = 64.0_f64 / 45.0_f64 * t17072;
    let t17075 = 32.0_f64 / 15.0_f64 * t5467 * t1893;
    let t17076 = t579 * t5207;
    let t17077 = 16.0_f64 / 45.0_f64 * t17076;
    let t17078 = t579 * t5563;
    let t17079 = 32.0_f64 / 15.0_f64 * t17078;
    let t17081 = 16.0_f64 / 5.0_f64 * t579 * t5179;
    let t17082 = t1783 * t1778;
    (t17069, t17071, t17073, t17075, t17077, t17079, t17081, t17082)
}
