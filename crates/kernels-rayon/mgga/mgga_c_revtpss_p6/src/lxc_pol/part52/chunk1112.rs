//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1112/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1112(t7535: f64, t9593: f64, t116: f64, t28651: f64, t2106: f64, t47672: f64, t2097: f64, t9990: f64, t3999: f64, t7506: f64, t198: f64, t7443: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102005 = t7535 * t9593;
    let t102019 = t28651 * t116;
    let t102070 = t2106 * t47672;
    let t102397 = t9990 * t2097;
    let t102622 = t3999 * t7506;
    let t102851 = t198 * t7443;
    (t102005, t102019, t102070, t102397, t102622, t102851)
}
