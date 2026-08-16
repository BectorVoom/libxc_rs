//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 729/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk729(t2704: f64, t2718: f64, t4518: f64, t4521: f64, t4524: f64, t4529: f64, t4531: f64, t4533: f64, t404: f64, t389: f64, t4510: f64, t1291: f64) -> (f64, f64) {
    let t4536 = -0.25319e1_f64 * t4518 + 0.16879333333333333333e1_f64 * t4521 - 0.19692555555555555555e1_f64 * t4524 - 0.93011851851851851854e0_f64 * t2704 + 0.13651666666666666667e0_f64 * t4529 - 0.27303333333333333333e0_f64 * t4531 - 0.3185388888888888889e0_f64 * t4533 - 0.36514074074074074075e0_f64 * t2718;
    let t4537 = t4536 * t404;
    let t4538 = t389 * t4537;
    let t4539 = 1.0_f64 * t4538;
    let t4540 = t4510 * t404;
    let t4541 = t1291 * t4540;
    (t4539, t4541)
}
