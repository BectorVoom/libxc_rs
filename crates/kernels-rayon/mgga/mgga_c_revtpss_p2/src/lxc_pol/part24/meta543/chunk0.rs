//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1603/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1603(t45: f64, t57: f64, t18272: f64, t22671: f64, t2375: f64, t39825: f64, t4377: f64, t5825: f64, t78: f64, t87107: f64, t87126: f64, t87145: f64, t18286: f64, t2382: f64, t39840: f64, t4384: f64, t81: f64, zeta_threshold: f64) -> (f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t87280 = piecewise3(t151, 0.0_f64, 40.0_f64 / 81.0_f64 * t39825 * t87145 - 16.0_f64 / 9.0_f64 * t18272 * t5825 + 4.0_f64 / 3.0_f64 * t2375 * t87107 + 16.0_f64 / 9.0_f64 * t4377 * t22671 + 4.0_f64 / 3.0_f64 * t78 * t87126);
    let t87292 = piecewise3(t155, 0.0_f64, 40.0_f64 / 81.0_f64 * t39840 * t87145 + 16.0_f64 / 9.0_f64 * t18286 * t5825 + 4.0_f64 / 3.0_f64 * t2382 * t87107 + 16.0_f64 / 9.0_f64 * t4384 * t22671 - 4.0_f64 / 3.0_f64 * t81 * t87126);
    (t87280, t87292)
}
