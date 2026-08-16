//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1126/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1126(t10867: f64, t239: f64, t8478: f64, t8484: f64, t124: f64, t800: f64, t815: f64, t886: f64, t32474: f64, t51076: f64, t7076: f64, t2453: f64, t8648: f64) -> (f64, f64, f64, f64, f64) {
    let t120097 = t8478 * t8484 * t10867 * t239;
    let t120106 = t815 * t800 * t124 * t886;
    let t120107 = t32474 * t120106;
    let t120108 = 0.13223814266738539448e-3_f64 * t120107;
    let t120110 = t7076 * t51076;
    let t120111 = t2453 * t8648 * t120110;
    (t120097, t120106, t120108, t120110, t120111)
}
