//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 908/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk908(t1275: f64, t1293: f64, t4659: f64, t1285: f64, t4661: f64, t4358: f64, t461: f64, t18486: f64, t18488: f64, t18491: f64, t18494: f64, t18500: f64, t18502: f64, t18504: f64, t18506: f64, t7236: f64, t7271: f64) -> (f64, f64, f64, f64, f64) {
    let t18515 = t1275 * t1275;
    let t18518 = 0.57894567559743977359e3_f64 * t4659 * t18515 * t1293;
    let t18527 = 0.3103500882342370105e4_f64 * t4659 * t1275 * t4661 * t1285;
    let t18528 = t4358 * t461;
    let t18529 = 96.0_f64 * t18528;
    let t18552 = -0.28769444444444444444e1_f64 * t18486 + 0.27618666666666666667e2_f64 * t18488 - 0.10229135802469135803e2_f64 * t18491 + 0.89504938271604938273e1_f64 * t18494 + 0.31310740740740740741e1_f64 * t7271 + 0.366775e-1_f64 * t18500 - 0.58684e0_f64 * t18502 + 0.65204444444444444445e0_f64 * t18504 + 0.5705388888888888889e0_f64 * t18506 + 0.13490888888888888889e1_f64 * t7236;
    (t18515, t18518, t18527, t18529, t18552)
}
