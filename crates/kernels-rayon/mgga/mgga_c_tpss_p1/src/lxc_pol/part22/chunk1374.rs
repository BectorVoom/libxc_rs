//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1374/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1374(t19349: f64, t62342: f64, t1791: f64, t65208: f64, t1675: f64, t18645: f64, t6090: f64, t18350: f64, t18673: f64, t19342: f64, t24290: f64, t62281: f64, t62285: f64, t62294: f64, t62339: f64, t62351: f64, t65169: f64, t65172: f64, t65175: f64, t65178: f64, t7690: f64) -> f64 {
    let t67369 = 160.0_f64 / 9.0_f64 * t19349 * t62342;
    let t67378 = t1791 * t65208;
    let t67385 = t1675 * t18645 * t6090;
    let t67387 = 80.0_f64 / 9.0_f64 * t62281 + 80.0_f64 / 9.0_f64 * t62285 - 20.0_f64 * t65178 * t62339 - t67369 + 20.0_f64 / 3.0_f64 * t65169 * t18673 + 20.0_f64 / 3.0_f64 * t65172 * t18673 + 20.0_f64 / 3.0_f64 * t65175 * t18673 + 20.0_f64 / 3.0_f64 * t19349 * t62351 + 20.0_f64 / 3.0_f64 * t18350 * t67378 - t62294 - 40.0_f64 * t7690 * t24290 * t19342 + 88.0_f64 / 27.0_f64 * t67385;
    t67387
}
