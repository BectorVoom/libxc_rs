//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1612/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1612(t45: f64, t57: f64, t18367: f64, t22671: f64, t2299: f64, t4328: f64, t5825: f64, t766: f64, t80: f64, t87107: f64, t87126: f64, t87145: f64, t18379: f64, t2306: f64, t4335: f64, t770: f64, t83: f64, zeta_threshold: f64) -> (f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t87529 = piecewise3(t151, 0.0_f64, -56.0_f64 / 81.0_f64 * t2299 * t87145 + 16.0_f64 / 9.0_f64 * t18367 * t5825 - 2.0_f64 / 3.0_f64 * t80 * t87107 - 8.0_f64 / 9.0_f64 * t4328 * t22671 + 2.0_f64 / 3.0_f64 * t766 * t87126);
    let t87541 = piecewise3(t155, 0.0_f64, -56.0_f64 / 81.0_f64 * t2306 * t87145 - 16.0_f64 / 9.0_f64 * t18379 * t5825 - 2.0_f64 / 3.0_f64 * t83 * t87107 - 8.0_f64 / 9.0_f64 * t4335 * t22671 - 2.0_f64 / 3.0_f64 * t770 * t87126);
    (t87529, t87541)
}
