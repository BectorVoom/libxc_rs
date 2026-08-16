//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1295/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1295(t18670: f64, t19408: f64, t1791: f64, t65157: f64, t65165: f64, t19342: f64, t62348: f64, t19349: f64, t62342: f64, t65208: f64, t1675: f64, t18645: f64, t6090: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t67337 = 80.0_f64 / 9.0_f64 * t18670 * t19408;
    let t67349 = t1791 * t65157;
    let t67352 = t1791 * t65165;
    let t67358 = 160.0_f64 / 3.0_f64 * t62348 * t19342;
    let t67369 = 160.0_f64 / 9.0_f64 * t19349 * t62342;
    let t67378 = t1791 * t65208;
    let t67385 = t1675 * t18645 * t6090;
    (t67337, t67349, t67352, t67358, t67369, t67378, t67385)
}
