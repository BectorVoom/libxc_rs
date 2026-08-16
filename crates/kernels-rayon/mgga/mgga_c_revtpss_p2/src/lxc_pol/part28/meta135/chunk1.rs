//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 746/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk746(t2988: f64, t973: f64, t2846: f64, t2904: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t2882: f64, t2890: f64, t2898: f64, t2900: f64, t2906: f64, t2910: f64, t2913: f64, t2916: f64) -> (f64, f64, f64, f64) {
    let t2989 = t2988 * t973;
    let t2994 = 0.40256666666666666667e0_f64 * t2846;
    let t3001 = 0.137975e0_f64 * t2904;
    let t3006 = -0.1294625e1_f64 * t2882 + 0.258925e1_f64 * t2890 + t2994 + 0.20128333333333333334e0_f64 * t2848 - 0.20128333333333333333e0_f64 * t2855 + 0.60385e0_f64 * t2860 - 0.301925e0_f64 * t2864 + 0.82524375e-1_f64 * t2898 + 0.16504875e0_f64 * t2900 + t3001 + 0.11038e0_f64 * t2906 - 0.27595e-1_f64 * t2910 + 0.16557e0_f64 * t2913 - 0.82785e-1_f64 * t2916;
    (t2989, t2994, t3001, t3006)
}
