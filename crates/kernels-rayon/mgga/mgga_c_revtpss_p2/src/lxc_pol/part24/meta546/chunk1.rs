//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1619/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1619(t6002: f64, t61037: f64, t61180: f64, t76979: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64, t39818: f64, t39823: f64, t40084: f64) -> (f64, f64, f64, f64) {
    let t87649 = 72.0_f64 * t61037 * t6002;
    let t87650 = 48.0_f64 * t61180;
    let t87651 = 48.0_f64 * t76979;
    let t87652 = -t39791 - t39795 + t87649 + t39799 + t39807 - t39813 + t87650 - t39818 - t39823 + t40084 + t87651;
    (t87649, t87650, t87651, t87652)
}
