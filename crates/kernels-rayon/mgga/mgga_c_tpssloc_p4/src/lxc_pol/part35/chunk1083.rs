//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1083/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1083(t22174: f64, t471: f64, t21762: f64, t248: f64, t3585: f64, t21510: f64, t4987: f64, t4582: f64, t1227: f64, t15503: f64, t15507: f64, t15569: f64, t15740: f64, t18357: f64, t18372: f64, t18376: f64, t18393: f64, t18972: f64, t18976: f64, t22154: f64, t22158: f64, t22162: f64, t22169: f64, t3577: f64, t488: f64, t5002: f64, t5005: f64, t5019: f64, t6192: f64, t6203: f64, t6221: f64, t6227: f64, t6232: f64) -> (f64, f64, f64) {
    let t22175 = t471 * t22174;
    let t22185 = t248 * t3585 * t21762;
    let t22196 = t4987 * t21510;
    let t22197 = t4582 * t22196;
    let t22202 = -t3577 * t22154 / 1536.0_f64 + 5.0_f64 / 4608.0_f64 * t3577 * t22158 - t3577 * t22162 / 1536.0_f64 + t15569 * t6192 / 144.0_f64 - t15740 * t6192 / 768.0_f64 + 19.0_f64 / 576.0_f64 * t22169 * t488 - 209.0_f64 / 2592.0_f64 * t22175 * t488 + t18357 / 768.0_f64 - t18372 / 1152.0_f64 + t18376 / 1536.0_f64 + t5002 * t6221 / 1024.0_f64 - t18393 / 1152.0_f64 + 5.0_f64 / 2304.0_f64 * t1227 * t22185 - t5019 * t6221 / 192.0_f64 - t15503 * t6227 / 96.0_f64 + t15507 * t6232 / 192.0_f64 + 5.0_f64 / 4608.0_f64 * t5005 * t6203 + 5.0_f64 / 4608.0_f64 * t1227 * t22197 + t18972 / 768.0_f64 + 5.0_f64 / 6912.0_f64 * t18976;
    (t22185, t22197, t22202)
}
