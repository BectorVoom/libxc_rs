//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 655/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk655(t741: f64, t9082: f64, t641: f64, t8786: f64, t746: f64, t719: f64, t8672: f64, t735: f64, t5284: f64, t9048: f64, t9052: f64, t9056: f64, t9059: f64, t9063: f64, t9067: f64, t9070: f64, t9073: f64, t9080: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9083 = t741 * t9082;
    let t9085 = t641 * t8786;
    let t9086 = t746 * t9085;
    let t9087 = t741 * t9086;
    let t9089 = t719 * t8672;
    let t9090 = t735 * t9089;
    let t9091 = t5284 * t9090;
    let t9093 = -t9048 / 12.0_f64 - t9052 / 128.0_f64 + 11.0_f64 / 18.0_f64 * t9056 - 2.0_f64 / 9.0_f64 * t9059 - t9063 / 256.0_f64 - t9067 / 576.0_f64 - t9070 / 24.0_f64 + t9073 / 96.0_f64 - 19.0_f64 / 144.0_f64 * t9080 + t9083 / 18.0_f64 - t9087 / 192.0_f64 + t9091 / 8.0_f64;
    (t9083, t9085, t9086, t9087, t9089, t9090, t9091, t9093)
}
