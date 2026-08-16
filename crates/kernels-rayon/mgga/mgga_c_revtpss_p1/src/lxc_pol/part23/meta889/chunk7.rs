//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2826/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2826(t45: f64, t14447: f64, t1490: f64, t18281: f64, t18367: f64, t19680: f64, t22671: f64, t22688: f64, t2299: f64, t4186: f64, t4328: f64, t5825: f64, t606: f64, t76397: f64, t766: f64, t80: f64, zeta_threshold: f64) -> f64 {
    let t151 = t45 <= zeta_threshold;
    let t76401 = piecewise3(t151, 0.0_f64, -56.0_f64 / 81.0_f64 * t2299 * t22688 * t606 + 8.0_f64 / 9.0_f64 * t18367 * t4186 + 8.0_f64 / 9.0_f64 * t1490 * t19680 - 2.0_f64 / 3.0_f64 * t14447 * t5825 - 2.0_f64 / 3.0_f64 * t4328 * t18281 - 2.0_f64 / 9.0_f64 * t80 * t22671 * t606 + 2.0_f64 / 3.0_f64 * t766 * t76397);
    t76401
}
