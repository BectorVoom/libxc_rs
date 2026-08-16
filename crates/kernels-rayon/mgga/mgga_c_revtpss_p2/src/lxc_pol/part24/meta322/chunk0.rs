//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1114/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1114(t22718: f64, t38: f64, t10389: f64, t10398: f64, t22671: f64, t22688: f64, t4227: f64, t4232: f64, t5825: f64, t633: f64, t637: f64, t77: f64) -> (f64, f64) {
    let t22719 = t38 * t22718;
    let t22738 = -280.0_f64 / 27.0_f64 * t10389 * t22688 + 28.0_f64 / 3.0_f64 * t4227 * t5825 - 4.0_f64 / 3.0_f64 * t633 * t22671 + 280.0_f64 / 27.0_f64 * t10398 * t22688 + 28.0_f64 / 3.0_f64 * t4232 * t5825 + 4.0_f64 / 3.0_f64 * t637 * t22671;
    let t22739 = t77 * t22738;
    (t22719, t22739)
}
