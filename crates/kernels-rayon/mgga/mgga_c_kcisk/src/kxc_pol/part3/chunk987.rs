//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 987/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk987(t14546: f64, t498: f64, t14545: f64, t14150: f64, t467: f64, t492: f64, t500: f64, t14400: f64, t14402: f64, t14514: f64, t14516: f64, t14519: f64, t14522: f64, t14525: f64, t14529: f64, t14532: f64, t14536: f64, t14538: f64, t14541: f64, t14543: f64) -> (f64, f64, f64) {
    let t14547 = t498 * t14546;
    let t14548 = t14545 * t14547;
    let t14550 = t14150 * t467;
    let t14551 = t14550 * t492;
    let t14552 = t14551 * t500;
    let t14554 = -3.0_f64 / 16.0_f64 * t14400 + 3.0_f64 / 256.0_f64 * t14402 + t14514 / 16.0_f64 - t14516 / 192.0_f64 + t14519 / 64.0_f64 - t14522 / 24.0_f64 + t14525 / 2.0_f64 + t14529 / 24.0_f64 - 3.0_f64 / 128.0_f64 * t14532 - t14536 / 192.0_f64 - 2.0_f64 / 3.0_f64 * t14538 - t14541 / 3.0_f64 + t14543 / 8.0_f64 + 3.0_f64 / 128.0_f64 * t14548 - t14552 / 256.0_f64;
    (t14548, t14552, t14554)
}
