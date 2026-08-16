//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2073/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2073(t25875: f64, t97676: f64, t97680: f64, t1444: f64, t5740: f64, t675: f64, t685: f64, t94395: f64, t14109: f64, t25900: f64, t94649: f64, t1892: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97682 = 0.51405703062096148812e-1_f64 * t25875 * t97676 * t97680;
    let t97685 = t5740 * t685 * t675 * t1444;
    let t97687 = 0.28912093960683998208e-1_f64 * t94395 * t97685;
    let t97688 = t14109 * t25900;
    let t97690 = 0.28912093960683998208e-1_f64 * t94395 * t97688;
    let t97698 = 0.51405703062096148812e-1_f64 * t94649 * t97688;
    let t97699 = t786 * t1892;
    (t97682, t97685, t97687, t97690, t97698, t97699)
}
