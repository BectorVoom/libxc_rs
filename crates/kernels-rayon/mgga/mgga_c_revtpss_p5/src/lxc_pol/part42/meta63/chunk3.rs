//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 381/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk381(t1032: f64, t460: f64, t472: f64, t474: f64, t1038: f64, t479: f64) -> (f64, f64, f64, f64, f64) {
    let t1241 = t460 * t1032;
    let t1242 = t472 * t472;
    let t1243 = 1.0_f64 / t1242;
    let t1244 = t1243 * t474;
    let t1245 = t479 * t1038;
    let t1246 = t1244 * t1245;
    (t1241, t1242, t1243, t1244, t1246)
}
