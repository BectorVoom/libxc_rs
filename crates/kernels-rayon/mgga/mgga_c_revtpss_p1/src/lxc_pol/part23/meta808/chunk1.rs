//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2643/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2643(t14991: f64, t50208: f64, t14485: f64, t14987: f64, t18657: f64, t213: f64, t14983: f64, t18392: f64, t262: f64, t18838: f64, t2411: f64, t18969: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t63094 = t50208 * t14991;
    let t63099 = t14987 * t14485;
    let t63103 = t213 * t18657;
    let t63109 = t14987 * t14983;
    let t63146 = t262 * t18392;
    let t63160 = t18838 * t2411;
    let t63240 = t698 * t18969;
    (t63094, t63099, t63103, t63109, t63146, t63160, t63240)
}
