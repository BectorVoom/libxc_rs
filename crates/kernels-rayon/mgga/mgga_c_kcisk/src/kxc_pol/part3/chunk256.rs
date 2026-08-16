//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 256/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk256(t1161: f64, t1184: f64, t1165: f64, t1176: f64, t1181: f64, t1188: f64) -> f64 {
    let t1205 = 0.301925e0_f64 * t1161;
    let t1208 = 0.82785e-1_f64 * t1184;
    let t1210 = 0.258925e1_f64 * t1176 - t1205 - 0.301925e0_f64 * t1165 + 0.16504875e0_f64 * t1181 - t1208 - 0.82785e-1_f64 * t1188;
    t1210
}
