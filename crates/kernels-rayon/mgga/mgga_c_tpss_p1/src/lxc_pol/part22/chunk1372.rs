//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1372/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1372(t18347: f64, t18649: f64, t19404: f64, t5785: f64, t6077: f64, t62247: f64, t62250: f64, t62311: f64, t62330: f64, t65285: f64, t65293: f64, t67326: f64, t67331: f64, t67333: f64, t67335: f64, t67337: f64) -> f64 {
    let t67342 = -5.0_f64 / 3.0_f64 * t62311 * t6077 - 10.0_f64 / 3.0_f64 * t18649 * t19404 + 10.0_f64 / 3.0_f64 * t62330 * t6077 - 5.0_f64 / 3.0_f64 * t5785 * t65285 + 10.0_f64 * t67326 * t18347 + t67331 + t67333 + t67335 + t67337 - 5.0_f64 / 3.0_f64 * t5785 * t65293 + 16.0_f64 / 9.0_f64 * t62247 - 8.0_f64 / 9.0_f64 * t62250;
    t67342
}
