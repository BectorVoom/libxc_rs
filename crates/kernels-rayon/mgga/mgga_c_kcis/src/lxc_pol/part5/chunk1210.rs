//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1210/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1210(t19763: f64, t3438: f64, t3437: f64, t20157: f64, t20160: f64, t20162: f64, t20165: f64, t20167: f64, t20170: f64, t20174: f64, t20176: f64, t20179: f64, t20181: f64, t20183: f64, t20186: f64, t20188: f64, t20192: f64, t20195: f64, t20198: f64, t20201: f64, t20203: f64) -> (f64, f64) {
    let t20205 = t3438 * t19763;
    let t20206 = t3437 * t20205;
    let t20208 = -t20157 / 16.0_f64 + t20160 / 4.0_f64 + t20162 / 96.0_f64 + t20165 / 6.0_f64 + t20167 / 8.0_f64 + t20170 / 288.0_f64 + t20174 / 256.0_f64 - t20176 / 192.0_f64 - t20179 / 24.0_f64 + t20181 / 24.0_f64 - t20183 / 8.0_f64 + t20186 / 27.0_f64 - t20188 / 192.0_f64 - t20192 / 192.0_f64 - t20195 / 48.0_f64 + t20198 / 576.0_f64 + t20201 / 192.0_f64 + t20203 / 18.0_f64 + t20206 / 192.0_f64;
    (t20206, t20208)
}
