//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1304/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1304(t14538: f64, t3792: f64, t51328: f64, t15240: f64, t54079: f64, t54230: f64, t11451: f64, t14031: f64, t54119: f64, t8983: f64, t3108: f64, t3133: f64, t54253: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56880 = t14538 * t51328 * t3792;
    let t56883 = t54079 * t15240;
    let t56885 = t54230 * t15240;
    let t56887 = t14031 * t11451;
    let t56889 = t54119 * t8983;
    let t56892 = t3108 * t54253 * t3133;
    (t56880, t56883, t56885, t56887, t56889, t56892)
}
