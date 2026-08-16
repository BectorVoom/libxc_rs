//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 842/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk842(t10003: f64, t2618: f64, t2623: f64, t2630: f64, t2635: f64, t2643: f64, t2681: f64, t2703: f64, t843: f64, t849: f64, t9967: f64, t9974: f64, t9978: f64, t9983: f64, t9986: f64, t9988: f64, t9990: f64, t9994: f64, t9997: f64) -> f64 {
    let t10006 = -t2618 * t2681 / 1024.0_f64 + t9967 * t2635 / 512.0_f64 - t9974 * t9978 / 512.0_f64 + t2630 * t9983 / 512.0_f64 + 7.0_f64 / 1536.0_f64 * t9986 - 35.0_f64 / 384.0_f64 * t9988 - t9990 * t849 / 256.0_f64 + 7.0_f64 / 192.0_f64 * t9994 - t843 * t9997 / 768.0_f64 + 5.0_f64 / 256.0_f64 * t2623 * t2703 + t2643 * t10003 / 256.0_f64;
    t10006
}
