//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 517/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk517(t3473: f64, t625: f64, t11: f64, t1743: f64, t2696: f64, t3467: f64, t3471: f64, t203: f64, t184: f64) -> (f64, f64, f64, f64, f64) {
    let t3474 = t625 * t3473;
    let t3475 = t11 * t3474;
    let t3477 = -t1743 - 0.12594444444444444445e-2_f64 * t2696 + 0.12594444444444444445e-2_f64 * t3467 - 0.37783333333333333334e-2_f64 * t3471 + 0.18891666666666666667e-2_f64 * t3475;
    let t3478 = t203 * t3477;
    let t3479 = t3478 * t184;
    (t3474, t3475, t3477, t3478, t3479)
}
