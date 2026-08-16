//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 839/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk839(t3205: f64, t329: f64, t838: f64, t332: f64, t6238: f64, t863: f64, t2079: f64, t2112: f64, t2153: f64, t328: f64, t6643: f64, t824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8801 = t329 * t838 * t3205;
    let t8903 = t863 * t6238 * t332;
    let t8944 = t2079 * t2112;
    let t8967 = t863 * t2153 * t838;
    let t8986 = t6643 * t328;
    let t8987 = t824 * t8986;
    (t8801, t8903, t8944, t8967, t8986, t8987)
}
