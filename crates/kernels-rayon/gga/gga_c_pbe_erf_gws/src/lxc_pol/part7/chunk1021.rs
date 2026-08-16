//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1021/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1021(t414: f64, t4601: f64, t1275: f64, t1293: f64, t4659: f64, t40: f64, t427: f64, t4742: f64, t1423: f64, t1438: f64, t1285: f64, t4661: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18513 = t414 * t4601;
    let t18514 = 48.0_f64 * t18513;
    let t18515 = t1275 * t1275;
    let t18518 = 0.57894567559743977359e3_f64 * t4659 * t18515 * t1293;
    let t18520 = t40 * t427 * t4742;
    let t18521 = 4.0_f64 * t18520;
    let t18522 = t1438 * t1423;
    let t18523 = 192.0_f64 * t18522;
    let t18527 = 0.3103500882342370105e4_f64 * t4659 * t1275 * t4661 * t1285;
    (t18514, t18515, t18518, t18521, t18523, t18527)
}
