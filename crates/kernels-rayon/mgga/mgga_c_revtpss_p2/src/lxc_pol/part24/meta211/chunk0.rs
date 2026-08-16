//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 951/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk951(t11015: f64, t787: f64, t781: f64, t9292: f64, t2410: f64, t261: f64, t3335: f64, t389: f64, t1077: f64, t225: f64, t268: f64, t271: f64, t7021: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11017 = 0.30356481678079769392e-1_f64 * t787 * t11015;
    let t11040 = 0.17073386770573548589e-1_f64 * t9292 * t781;
    let t11064 = 1.0_f64 / t2410 / t261;
    let t11108 = 1.0_f64 / t3335 / t389;
    let t11119 = t1077 * t1077;
    let t11120 = 1.0_f64 / t11119;
    let t11121 = t225 * t11120;
    let t11132 = t268 * t7021 * t271;
    (t11017, t11040, t11064, t11108, t11119, t11121, t11132)
}
