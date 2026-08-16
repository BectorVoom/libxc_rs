//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1027/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1027(t42267: f64, t42272: f64, t42275: f64, t42278: f64, t42282: f64, t42284: f64, t42288: f64, t42292: f64, t42298: f64, t42305: f64, t42309: f64, t42312: f64, t42315: f64, t42340: f64, t48121: f64, t48124: f64, t48127: f64, t48131: f64, t48134: f64, t48137: f64) -> f64 {
    let t50902 = t42267 + t42272 + t42275 + t42278 + 0.51123901271894332901e0_f64 * t48121 + 0.14300195980740170668e1_f64 * t48124 + t42282 - t42284 - t42288 - t42292 - t42298 + 0.95334639871601137787e0_f64 * t48127 + t42305 - t42309 - t42312 + 0.23005755572352449806e2_f64 * t48131 + 0.23005755572352449806e2_f64 * t48134 + 0.23005755572352449806e2_f64 * t48137 - t42315 + t42340;
    t50902
}
