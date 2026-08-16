//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1011/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1011(t12526: f64, t3295: f64, t3332: f64, t9445: f64, t2147: f64, t10819: f64, t11768: f64, t12158: f64, t12162: f64, t12163: f64, t12164: f64, t12166: f64, t12167: f64, t12512: f64, t12515: f64, t12518: f64, t12521: f64, t12524: f64) -> (f64, f64) {
    let t12527 = t3295 * t12526;
    let t12529 = t3332 * t9445;
    let t12530 = t2147 * t12529;
    let t12532 = 0.86682217400542685632e-1_f64 * t12512 - t12158 + t12162 + t12163 - t12164 - 0.97574405393827830186e-2_f64 * t11768 - t12166 + t12167 - 0.86682217400542685632e-1_f64 * t12515 - 0.43341108700271342816e-1_f64 * t12518 - 0.43341108700271342816e-1_f64 * t12521 - 0.27439371595564631661e-1_f64 * t12524 - 0.27439371595564631661e-1_f64 * t12527 - t10819 + 0.21831846657716620896e-2_f64 * t12530;
    (t12529, t12532)
}
