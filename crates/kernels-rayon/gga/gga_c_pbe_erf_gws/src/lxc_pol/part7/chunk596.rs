//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 596/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk596(t242: f64, t4551: f64, t1597: f64, t700: f64, t1383: f64, t528: f64, t532: f64, t4358: f64, t35: f64, t413: f64) -> (f64, f64, f64, f64, f64) {
    let t4552 = t4551 * t242;
    let t4554 = t1597 * t700;
    let t4557 = 0.25128846160651320563e0_f64 * t528 * t1383;
    let t4558 = 12.0_f64 * t532;
    let t4559 = 36.0_f64 * t4358;
    let t4560 = t35 * t413;
    let t4561 = 24.0_f64 * t4560;
    let t4562 = t4558 - t4559 + t4561;
    (t4552, t4554, t4557, t4560, t4562)
}
