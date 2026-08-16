//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1997/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1997(t101886: f64, t108733: f64, t108737: f64, t108745: f64, t108807: f64, t108810: f64, t108813: f64, t2048: f64, t26187: f64, t28105: f64, t28109: f64, t28602: f64, t29538: f64, t29544: f64, t29548: f64, t7343: f64, t7352: f64, t7706: f64) -> f64 {
    let t109970 = -10.0_f64 / 3.0_f64 * t101886 * t7706 - 10.0_f64 / 3.0_f64 * t28602 * t28105 - 10.0_f64 / 3.0_f64 * t28602 * t28109 - 4.0_f64 / 3.0_f64 * t108807 * t2048 - 4.0_f64 / 3.0_f64 * t108810 * t2048 - 4.0_f64 / 3.0_f64 * t108813 * t2048 - 4.0_f64 / 3.0_f64 * t29538 * t7352 - 10.0_f64 / 3.0_f64 * t26187 * t29544 - 10.0_f64 / 3.0_f64 * t7343 * t108733 - 10.0_f64 / 3.0_f64 * t7343 * t108737 - 5.0_f64 / 3.0_f64 * t26187 * t29548 - 5.0_f64 / 3.0_f64 * t7343 * t108745;
    t109970
}
