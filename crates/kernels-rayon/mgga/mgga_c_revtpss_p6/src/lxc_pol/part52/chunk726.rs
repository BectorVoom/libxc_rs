//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 726/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk726(t225: f64, t7506: f64, t2097: f64, t213: f64, t2102: f64, t72: f64, t686: f64) -> (f64, f64, f64, f64) {
    let t7507 = t7506 * t225;
    let t7511 = t213 * t2097;
    let t7514 = t2102 * t72;
    let t7515 = t7514 * t686;
    (t7507, t7511, t7514, t7515)
}
