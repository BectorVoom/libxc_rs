//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 447/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk447(t5: f64, t2240: f64, t2242: f64, t2247: f64, t2248: f64, t2315: f64, t603: f64, t644: f64, t91: f64, t117: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t2319 = piecewise3(t8, 0.0_f64, t2240 * t91 - 8.0_f64 * t2242 * t644 + 20.0_f64 * t2247 * t2248 - 4.0_f64 * t2315 * t603);
    let t2320 = t2319 * t117;
    (t2319, t2320)
}
