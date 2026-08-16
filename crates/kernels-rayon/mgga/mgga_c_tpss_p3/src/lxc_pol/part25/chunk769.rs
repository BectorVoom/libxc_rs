//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 769/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk769(t3054: f64, t5248: f64, t1112: f64, t242: f64, t450: f64, t1501: f64, t1562: f64, t3068: f64, t3097: f64, t5064: f64, t1111: f64, t1125: f64, t1575: f64, t3052: f64, t3067: f64, t3080: f64, t4210: f64, t4265: f64, t444: f64, t463: f64, t5223: f64, t5231: f64, t5235: f64, t5239: f64, t5245: f64) -> (f64, f64, f64, f64) {
    let t5249 = t5248 * t3054;
    let t5250 = t1112 * t5249;
    let t5251 = t242 * t5250;
    let t5254 = t5248 * t450;
    let t5255 = t1112 * t5254;
    let t5256 = t242 * t5255;
    let t5261 = t1562 * t1501;
    let t5262 = t3068 * t5261;
    let t5265 = t3097 * t5064;
    let t5266 = t242 * t5265;
    let t5269 = 11.0_f64 / 108.0_f64 * t5223 * t444 - t4210 / 54.0_f64 + 19.0_f64 / 1728.0_f64 * t5231 * t463 - t1125 * t5235 / 4608.0_f64 - t1125 * t5239 / 2304.0_f64 + t1111 * t5245 / 3072.0_f64 + t3052 * t5251 / 1536.0_f64 - t3080 * t5256 / 3072.0_f64 + t4265 * t1575 / 432.0_f64 - t3067 * t5262 / 2304.0_f64 + 5.0_f64 / 13824.0_f64 * t1125 * t5266;
    (t5249, t5254, t5261, t5269)
}
