//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3308/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3308(t1448: f64, t6836: f64, t13600: f64, t22466: f64, t39799: f64, t39807: f64, t39813: f64, t4139: f64, t47059: f64, t48271: f64, t5536: f64, t5627: f64, t6816: f64, t85913: f64, t85914: f64, t85918: f64, t85919: f64) -> (f64, f64) {
    let t86753 = t6836 * t1448;
    let t86764 = 9.0_f64 * t13600 * t4139 * t6816 - 18.0_f64 * t22466 * t5536 * t5627 + t39799 + t39807 - t39813 + t47059 + t48271 - t85913 + t85914 - t85918 - t85919;
    (t86753, t86764)
}
