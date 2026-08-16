//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 865/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk865(t4882: f64, t5137: f64, t639: f64, t2735: f64, t662: f64, t211: f64, t5129: f64, t5529: f64, t587: f64, t4972: f64, t5125: f64, t4963: f64, t7669: f64) -> (f64, f64, f64, f64, f64) {
    let t16629 = t639 * t5137 * t4882;
    let t16630 = 64.0_f64 / 45.0_f64 * t16629;
    let t16631 = t2735 * t662;
    let t16632 = t211 * t16631;
    let t16633 = 64.0_f64 / 405.0_f64 * t16632;
    let t16635 = t587 * t5129 * t5529;
    let t16636 = 32.0_f64 / 45.0_f64 * t16635;
    let t16638 = t587 * t5125 * t4972;
    let t16639 = 64.0_f64 / 45.0_f64 * t16638;
    let t16641 = t587 * t7669 * t4963;
    (t16630, t16633, t16636, t16639, t16641)
}
