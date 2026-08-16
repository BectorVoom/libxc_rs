//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 833/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk833(t1031: f64, t3800: f64, t498: f64, t1207: f64, t1275: f64, t4171: f64, t602: f64, t1466: f64, t2246: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11238 = t1031 * t1031;
    let t11239 = 1.0_f64 / t11238;
    let t12587 = 1.0_f64 / t3800 / t498;
    let t12625 = t1207 * t1207;
    let t12626 = 1.0_f64 / t12625;
    let t13180 = t1275 * t1275;
    let t13181 = 1.0_f64 / t13180;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    (t11239, t12587, t12626, t13181, t13269, t13272)
}
