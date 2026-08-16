//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 926/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk926(t547: f64, t9714: f64, t807: f64, t9646: f64, t2236: f64, t66: f64) -> (f64, f64, f64, f64) {
    let t9715 = t547 * t9714;
    let t9716 = t807 * t9715;
    let t9718 = t9646 * t547;
    let t9720 = 1.0_f64 / t66 / t2236;
    (t9715, t9716, t9718, t9720)
}
