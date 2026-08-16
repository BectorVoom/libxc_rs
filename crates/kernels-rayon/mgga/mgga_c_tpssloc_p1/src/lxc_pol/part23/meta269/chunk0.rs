//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 946/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk946(t1824: f64, t6387: f64, t12250: f64, t1343: f64, t820: f64, t3792: f64, t119: f64, t20416: f64, t210: f64, t12291: f64, t12330: f64, t12335: f64, t1315: f64, t16341: f64, t16350: f64, t19915: f64, t19917: f64, t19933: f64, t3790: f64, t5235: f64, t6417: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20489 = t6387 * t1824;
    let t20490 = t20489 * t12250;
    let t20492 = t1343 * t820 * t20490;
    let t20495 = t20489 * t3792;
    let t20497 = t1343 * t820 * t20495;
    let t20500 = t119 * t20416;
    let t20501 = t210 * t20500;
    let t20508 = -35.0_f64 / 72.0_f64 * t16341 - t5235 * t6417 / 1024.0_f64 - t12291 * t20492 / 512.0_f64 + t3790 * t20497 / 512.0_f64 - t1315 * t20501 / 48.0_f64 + 119.0_f64 / 4608.0_f64 * t16350 - t12330 - t12335 + 7.0_f64 / 1536.0_f64 * t19915 + 7.0_f64 / 1536.0_f64 * t19917 + 7.0_f64 / 192.0_f64 * t19933;
    (t20489, t20490, t20492, t20495, t20497, t20500, t20501, t20508)
}
