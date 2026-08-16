//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 844/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk844(t4348: f64, t4498: f64, t4502: f64, t4505: f64, t4512: f64, t4538: f64, t4541: f64, t4744: f64, t16329: f64, t4344: f64, t4381: f64, t4545: f64, t4547: f64, t4602: f64, t6068: f64, t6839: f64, t6841: f64) -> f64 {
    let t16331 = 0.73024584604562962965e1_f64 * t4348;
    let t16334 = 48.0_f64 * t4498;
    let t16335 = 0.19298189186581325787e3_f64 * t4502;
    let t16336 = 24.0_f64 * t4505;
    let t16337 = 0.38596378373162651572e3_f64 * t4512;
    let t16338 = 4.0_f64 * t4538;
    let t16340 = 24.0_f64 * t4541;
    let t16345 = 4.0_f64 * t4744;
    let t16346 = t16329 - 0.49291594608080000001e1_f64 * t4344 - t16331 + 12.0_f64 * t4381 + 4.0_f64 * t6839 + t16334 + t16335 - t16336 - t16337 + t16338 - 36.0_f64 * t6841 + t16340 - 0.75926915593978166528e1_f64 * t4545 - 48.0_f64 * t4547 - 4.0_f64 * t6068 + 12.0_f64 * t4602 + t16345;
    t16346
}
