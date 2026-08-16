//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1021/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1021(t125: f64, t886: f64, t246: f64, t244: f64, t31838: f64, t239: f64, t2718: f64, t8484: f64, t8478: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31839 = t125 * t886;
    let t31840 = t246 * t31839;
    let t31841 = t244 * t31840;
    let t31842 = t31838 * t31841;
    let t31844 = t2718 * t239;
    let t31845 = t8484 * t31844;
    let t31846 = t8478 * t31845;
    (t31840, t31841, t31842, t31844, t31845, t31846)
}
