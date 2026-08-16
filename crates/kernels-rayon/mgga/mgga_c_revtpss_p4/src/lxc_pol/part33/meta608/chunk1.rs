//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2035/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2035(t13790: f64, t4102: f64, t685: f64, t72: f64, t25875: f64, t97676: f64, t1444: f64, t5740: f64, t675: f64, t94395: f64, t14109: f64, t25900: f64) -> (f64, f64, f64, f64, f64) {
    let t97680 = t13790 * t72 * t685 * t4102;
    let t97682 = 0.51405703062096148812e-1_f64 * t25875 * t97676 * t97680;
    let t97685 = t5740 * t685 * t675 * t1444;
    let t97687 = 0.28912093960683998208e-1_f64 * t94395 * t97685;
    let t97688 = t14109 * t25900;
    (t97680, t97682, t97685, t97687, t97688)
}
