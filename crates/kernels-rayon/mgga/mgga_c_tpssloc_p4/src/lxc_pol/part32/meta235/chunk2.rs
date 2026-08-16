//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1063/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1063(t3508: f64, t6224: f64, t1214: f64, t248: f64, t475: f64, t1213: f64, t1227: f64, t1737: f64, t1748: f64, t3506: f64, t3515: f64, t3542: f64, t3547: f64, t467: f64, t5005: f64, t5019: f64, t5024: f64, t5036: f64, t5041: f64, t6109: f64, t6203: f64, t6207: f64, t6211: f64, t6221: f64) -> (f64, f64, f64, f64, f64) {
    let t6225 = t6224 * t3508;
    let t6227 = t248 * t1214 * t6225;
    let t6230 = t6224 * t475;
    let t6232 = t248 * t1214 * t6230;
    let t6237 = -t5005 * t1748 / 2304.0_f64 - t5019 * t1737 / 288.0_f64 + 5.0_f64 / 13824.0_f64 * t1227 * t6203 - t1227 * t6207 / 4608.0_f64 - t1227 * t6211 / 2304.0_f64 - t5036 / 54.0_f64 + 11.0_f64 / 108.0_f64 * t6109 * t467 - t5041 / 432.0_f64 - t3542 + t1213 * t6221 / 3072.0_f64 + t3506 * t6227 / 1536.0_f64 - t3515 * t6232 / 3072.0_f64 + t5024 * t1748 / 432.0_f64 - t3547;
    (t6225, t6227, t6230, t6232, t6237)
}
