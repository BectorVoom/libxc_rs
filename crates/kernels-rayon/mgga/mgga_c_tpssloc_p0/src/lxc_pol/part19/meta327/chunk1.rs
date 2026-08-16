//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1163/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1163(t12012: f64, t550: f64, t120: f64, t12177: f64, t12371: f64, t16398: f64, t12283: f64, t12426: f64, t12250: f64, t1307: f64, t3850: f64, t12291: f64, t12368: f64, t12397: f64, t12419: f64, t12420: f64, t1341: f64, t1343: f64, t1352: f64, t16233: f64, t16305: f64, t3790: f64, t3803: f64, t3805: f64, t3806: f64, t3807: f64, t3853: f64, t40148: f64, t40153: f64, t40160: f64, t40162: f64, t40168: f64, t40169: f64, t820: f64) -> (f64, f64, f64) {
    let t40178 = t550 * t12012;
    let t40183 = t120 * t12177;
    let t40188 = t16398 * t12371;
    let t40190 = t12283 * t12426;
    let t40192 = t12250 * t1307;
    let t40197 = t1307 * t3850;
    let t40204 = -3.0_f64 / 256.0_f64 * t12291 * t1343 * t820 * t40148 - t1341 * t1343 * t820 * t40153 / 3072.0_f64 + 119.0_f64 / 1152.0_f64 * t40160 + 7.0_f64 / 1536.0_f64 * t3790 * t1343 * t820 * t40162 + 5.0_f64 / 32.0_f64 * t3803 * t40168 * t3806 * t40169 - 5.0_f64 / 128.0_f64 * t3803 * t12419 * t12368 * t12420 + t3803 * t3805 * t3806 * t40178 / 192.0_f64 + t3803 * t3805 * t40183 * t3807 / 192.0_f64 + 7.0_f64 / 48.0_f64 * t40188 - 7.0_f64 / 96.0_f64 * t40190 + t16233 * t3805 * t40183 * t40192 / 32.0_f64 + t3803 * t16305 * t1352 * t40197 / 64.0_f64 - t12397 * t3853 / 512.0_f64;
    (t40183, t40197, t40204)
}
