//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 946/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk946(t2564: f64, t2567: f64, t268: f64, t675: f64) -> f64 {
    let t9333 = 0.10685e0_f64 * t268 * t675 * t2564 * t2567;
    t9333
}
