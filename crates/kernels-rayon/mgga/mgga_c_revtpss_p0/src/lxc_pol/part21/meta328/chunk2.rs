//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1628/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1628(t3259: f64, t342: f64, t992: f64, t338: f64) -> (f64, f64, f64, f64) {
    let t11195 = t342 * t3259;
    let t11198 = t992 * t992;
    let t11199 = 1.0_f64 / t11198;
    let t11200 = t338 * t11199;
    (t11195, t11198, t11199, t11200)
}
