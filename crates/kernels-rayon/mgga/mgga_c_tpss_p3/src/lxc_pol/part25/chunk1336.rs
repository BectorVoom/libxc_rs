//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1336/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1336(t6308: f64, t645: f64, t4637: f64, t5798: f64, t19349: f64, t20264: f64, t62259: f64, t62262: f64, t65169: f64, t65172: f64, t65175: f64, t67331: f64, t67333: f64, t67335: f64, t67337: f64, t67349: f64, t67358: f64, t67369: f64) -> (f64, f64, f64) {
    let t71344 = t6308 * t645;
    let t71374 = t5798 * t4637;
    let t71386 = t67331 + t67333 + t67335 + t67337 + 88.0_f64 / 27.0_f64 * t62259 + 88.0_f64 / 27.0_f64 * t62262 + 20.0_f64 / 3.0_f64 * t65169 * t20264 + 20.0_f64 / 3.0_f64 * t65172 * t20264 + 20.0_f64 / 3.0_f64 * t65175 * t20264 + 20.0_f64 / 3.0_f64 * t19349 * t67349 - t67358 - t67369;
    (t71344, t71374, t71386)
}
