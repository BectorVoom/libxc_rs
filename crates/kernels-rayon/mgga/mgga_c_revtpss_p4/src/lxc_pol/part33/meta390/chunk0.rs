//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1440/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1440(t1729: f64, t2439: f64, t5098: f64, t698: f64, t16708: f64, t16710: f64, t16712: f64, t5095: f64, t3523: f64, t5180: f64, t1737: f64, t3451: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16876 = t2439 * t1729;
    let t16892 = t698 * t5098;
    let t16893 = 0.21908444444444444444e0_f64 * t16892;
    let t16915 = 4.0_f64 / 27.0_f64 * t16708;
    let t16916 = 4.0_f64 / 9.0_f64 * t16710;
    let t16917 = 2.0_f64 / 9.0_f64 * t16712;
    let t16929 = 0.39862222222222222222e0_f64 * t16710;
    let t16931 = t698 * t5095;
    let t16988 = t5180 * t3523;
    let t17010 = 0.2283111111111111111e-1_f64 * t16710;
    let t17011 = 0.11415555555555555555e-1_f64 * t16712;
    let t17023 = t1737 * t3451;
    (t16876, t16892, t16893, t16915, t16916, t16917, t16929, t16931, t16988, t17010, t17011, t17023)
}
