//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 530/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk530(t10177: f64, t6320: f64, t2268: f64, t10150: f64, t10153: f64, t10157: f64, t10162: f64, t10165: f64, t10169: f64, t10172: f64, t10176: f64, t1358: f64, t9089: f64, t9092: f64, t9094: f64, t9147: f64, t9149: f64) -> f64 {
    let t10178 = t6320 * t10177;
    let t10180 = 0.17073003981405689759e0_f64 * t2268 * t10178;
    let t10181 = t10150 + 0.56910013271352299198e-1_f64 * t2268 * t10153 - 0.85365019907028448797e-1_f64 * t2268 * t10157 - t10162 + t10165 - t10169 - 0.31616674039640166221e-2_f64 * t1358 * t10172 - t9089 + t9092 - t9094 + t10176 - t10180 + t9147 - t9149;
    t10181
}
