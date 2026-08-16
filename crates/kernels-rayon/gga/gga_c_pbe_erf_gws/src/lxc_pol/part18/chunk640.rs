//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 640/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk640(t1691: f64, t3465: f64, t11: f64, t1697: f64, t3351: f64, t625: f64, t3354: f64, t626: f64, t1743: f64, t2696: f64, t203: f64, t184: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3466 = t1691 * t3465;
    let t3467 = t11 * t3466;
    let t3469 = t1697 * t3351;
    let t3470 = t625 * t3469;
    let t3471 = t11 * t3470;
    let t3473 = t626 * t3354;
    let t3474 = t625 * t3473;
    let t3475 = t11 * t3474;
    let t3477 = -t1743 - 0.12594444444444444445e-2_f64 * t2696 + 0.12594444444444444445e-2_f64 * t3467 - 0.37783333333333333334e-2_f64 * t3471 + 0.18891666666666666667e-2_f64 * t3475;
    let t3478 = t203 * t3477;
    let t3479 = t3478 * t184;
    (t3466, t3467, t3469, t3470, t3471, t3473, t3474, t3475, t3477, t3478, t3479)
}
