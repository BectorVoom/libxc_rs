//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1023/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1023(t414: f64, t4832: f64, t1399: f64, t4749: f64, t409: f64, t4601: f64, t1425: f64, t1438: f64, t18486: f64, t18488: f64, t18491: f64, t18494: f64, t18500: f64, t18502: f64, t18504: f64, t18506: f64, t7236: f64, t7271: f64) -> (f64, f64, f64, f64, f64) {
    let t18534 = t414 * t4832;
    let t18535 = 16.0_f64 * t18534;
    let t18536 = t1399 * t4749;
    let t18537 = 0.4155781415850207192e3_f64 * t18536;
    let t18538 = t409 * t4601;
    let t18539 = 48.0_f64 * t18538;
    let t18540 = t1438 * t1425;
    let t18541 = 384.0_f64 * t18540;
    let t18552 = -0.28769444444444444444e1_f64 * t18486 + 0.27618666666666666667e2_f64 * t18488 - 0.10229135802469135803e2_f64 * t18491 + 0.89504938271604938273e1_f64 * t18494 + 0.31310740740740740741e1_f64 * t7271 + 0.366775e-1_f64 * t18500 - 0.58684e0_f64 * t18502 + 0.65204444444444444445e0_f64 * t18504 + 0.5705388888888888889e0_f64 * t18506 + 0.13490888888888888889e1_f64 * t7236;
    (t18535, t18537, t18539, t18541, t18552)
}
