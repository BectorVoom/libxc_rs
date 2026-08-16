//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 669/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk669(t5335: f64, t561: f64, t1879: f64, t1882: f64, t1735: f64, t2730: f64, t1748: f64, t202: f64, t184: f64, t619: f64, t1871: f64, t582: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5337 = 4.0_f64 / 15.0_f64 * t561 * t5335;
    let t5338 = t1879 * t1882;
    let t5339 = 16.0_f64 / 15.0_f64 * t5338;
    let t5341 = 4.0_f64 / 5.0_f64 * t2730 * t1735;
    let t5342 = t202 * t1748;
    let t5343 = t5342 * t184;
    let t5345 = 4.0_f64 / 5.0_f64 * t5343 * t619;
    let t5346 = t582 * t1871;
    (t5337, t5339, t5341, t5342, t5343, t5345, t5346)
}
