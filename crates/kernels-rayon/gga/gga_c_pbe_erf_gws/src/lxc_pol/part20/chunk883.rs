//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 883/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk883(t1: f64, t3360: f64, t467: f64, t6906: f64, t3342: f64, t4351: f64, t418: f64, t1351: f64, t2477: f64, t1523: f64, t3346: f64, t4358: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9762 = t3360 * t1;
    let t9763 = t9762 * t467;
    let t9764 = 0.18311555036753159941e-3_f64 * t9763;
    let t9765 = 0.13692109613355555556e1_f64 * t6906;
    let t9778 = t4351 * t3342;
    let t9779 = t9778 * t418;
    let t9781 = t2477 * t1351;
    let t9783 = t1523 * t3346;
    let t9784 = t9783 * t418;
    let t9788 = -2.0_f64 * t532 - 6.0_f64 * t4358;
    (t9764, t9765, t9779, t9781, t9784, t9788)
}
