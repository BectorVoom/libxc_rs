//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 215/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk215(t138: f64, t125: f64, t126: f64, t67: f64, t117: f64, t120: f64) -> (f64, f64, f64, f64, f64) {
    let t681 = t138 * t138;
    let t682 = 1.0_f64 / t681;
    let t683 = t125 * t682;
    let t685 = 1.0_f64 / t126 * t67;
    let t686 = t117 * t120;
    (t681, t682, t683, t685, t686)
}
