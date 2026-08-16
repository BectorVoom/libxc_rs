//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 945/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk945(t1730: f64, t5171: f64, t5163: f64, t582: f64, t616: f64, t5004: f64, t5480: f64, t639: f64, t1631: f64, t5470: f64, t1627: f64, t5477: f64) -> (f64, f64, f64, f64, f64) {
    let t17558 = t1730 * t5171;
    let t17559 = 32.0_f64 / 15.0_f64 * t17558;
    let t17561 = t616 * t582 * t5163;
    let t17562 = 32.0_f64 / 45.0_f64 * t17561;
    let t17564 = t639 * t5480 * t5004;
    let t17565 = 64.0_f64 / 27.0_f64 * t17564;
    let t17566 = t5470 * t1631;
    let t17567 = 32.0_f64 / 45.0_f64 * t17566;
    let t17568 = t1627 * t5477;
    (t17559, t17562, t17565, t17567, t17568)
}
