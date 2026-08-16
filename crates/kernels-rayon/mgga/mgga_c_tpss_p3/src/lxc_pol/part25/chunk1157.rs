//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1157/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1157(t1113: f64, t1141: f64, t1143: f64, t1581: f64, t15930: f64, t15956: f64, t15960: f64, t15964: f64, t15968: f64, t15975: f64, t15979: f64, t15989: f64, t15992: f64, t15999: f64, t16004: f64, t220: f64, t3124: f64, t3126: f64, t3138: f64, t3139: f64, t4303: f64, t4314: f64, t468: f64, t5270: f64, t5279: f64, t5283: f64, t5287: f64, t9749: f64, t9764: f64, t9787: f64) -> f64 {
    let t16012 = t1113 * t1141 * t1143 * t5270 + 2.0_f64 * t1141 * t1143 * t15975 + 2.0_f64 * t1141 * t1143 * t15979 + t1141 * t1143 * t15989 + t1141 * t1143 * t15992 + 4.0_f64 * t1581 * t15964 * t3124 - 2.0_f64 * t1581 * t15999 * t3138 + t15930 * t220 * t468 + 6.0_f64 * t15956 * t5279 * t9749 + 2.0_f64 * t15960 * t3124 * t3126 - t15960 * t3138 * t3139 - 6.0_f64 * t15968 * t5279 * t9764 + t16004 * t5279 * t9787 + 4.0_f64 * t3124 * t4303 * t5283 + 2.0_f64 * t3124 * t4303 * t5287 - 2.0_f64 * t3138 * t4314 * t5283 - t3138 * t4314 * t5287;
    t16012
}
