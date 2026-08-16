//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 960/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk960(t5463: f64, t649: f64, t1816: f64, t639: f64, t1726: f64, t1783: f64, t1683: f64, t1798: f64, t5343: f64, t185: f64, t5274: f64, t582: f64) -> (f64, f64, f64, f64, f64) {
    let t17791 = t5463 * t649;
    let t17793 = t639 * t17791 * t1816;
    let t17794 = 32.0_f64 / 135.0_f64 * t17793;
    let t17796 = 8.0_f64 / 5.0_f64 * t1783 * t1726;
    let t17797 = t1783 * t1683;
    let t17798 = 32.0_f64 / 15.0_f64 * t17797;
    let t17799 = t5343 * t1798;
    let t17800 = 32.0_f64 / 15.0_f64 * t17799;
    let t17802 = t185 * t582 * t5274;
    (t17794, t17796, t17798, t17800, t17802)
}
