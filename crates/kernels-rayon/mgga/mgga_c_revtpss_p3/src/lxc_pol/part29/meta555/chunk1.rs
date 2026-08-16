//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1897/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1897(t1353: f64, t28198: f64, t13790: f64, t4102: f64, t685: f64, t72: f64, t1444: f64, t5740: f64, t675: f64, t14109: f64, t25900: f64, t1892: f64, t786: f64) -> (f64, f64, f64, f64, f64) {
    let t97654 = t28198 * t1353;
    let t97680 = t13790 * t72 * t685 * t4102;
    let t97685 = t5740 * t685 * t675 * t1444;
    let t97688 = t14109 * t25900;
    let t97699 = t786 * t1892;
    (t97654, t97680, t97685, t97688, t97699)
}
