//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1147/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1147(t3392: f64, t3479: f64, t12480: f64, t1820: f64, t1821: f64, t995: f64, t3493: f64, t3555: f64, t11032: f64, t3519: f64, t3523: f64, t12639: f64, t2612: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48261 = 8.0_f64 / 5.0_f64 * t3479 * t3392;
    let t48265 = 32.0_f64 / 45.0_f64 * t1820 * t1821 * t12480 * t995;
    let t48267 = 8.0_f64 / 5.0_f64 * t3493 * t3555;
    let t48270 = 8.0_f64 / 15.0_f64 * t11032 * t3519;
    let t48272 = 8.0_f64 / 9.0_f64 * t11032 * t3523;
    let t48274 = 16.0_f64 / 45.0_f64 * t2612 * t12639;
    (t48261, t48265, t48267, t48270, t48272, t48274)
}
