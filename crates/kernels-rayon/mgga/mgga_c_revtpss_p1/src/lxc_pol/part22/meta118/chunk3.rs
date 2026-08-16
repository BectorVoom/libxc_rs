//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 805/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk805(t2847: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t291: f64, t910: f64, t914: f64) -> (f64, f64, f64) {
    let t2866 = t2847 + 0.11872222222222222222e-1_f64 * t2848 - 0.11872222222222222222e-1_f64 * t2855 + 0.35616666666666666666e-1_f64 * t2860 - 0.17808333333333333333e-1_f64 * t2864;
    let t2868 = 0.621814e-1_f64 * t2866 * t291;
    let t2869 = t910 * t914;
    (t2866, t2868, t2869)
}
