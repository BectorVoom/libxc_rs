//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 564/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk564(t2846: f64, t689: f64, t907: f64) -> (f64, f64) {
    let t2847 = 0.23744444444444444444e-1_f64 * t2846;
    let t2848 = t689 * t907;
    (t2847, t2848)
}
