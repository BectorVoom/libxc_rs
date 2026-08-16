//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1440/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1440(t300: f64, t44115: f64, t44138: f64, t44198: f64, t44366: f64, t1164: f64, t3396: f64, t3422: f64, t43994: f64, t43997: f64, t44000: f64, t44002: f64, t44006: f64, t44072: f64, t44080: f64, t44082: f64, t44085: f64, t44089: f64, t44092: f64) -> (f64, f64, f64) {
    let t44369 = t300 * (t44115 + t44138 + t44198 + t44366);
    let t44372 = 0.21053605041484726346e2_f64 * t1164 * t3422 * t3396;
    let t44373 = t43994 - t43997 - t44000 + t44002 + t44006 + t44072 + t44080 + t44082 - t44085 - t44089 + t44092 + t44369 - t44372;
    (t44369, t44372, t44373)
}
