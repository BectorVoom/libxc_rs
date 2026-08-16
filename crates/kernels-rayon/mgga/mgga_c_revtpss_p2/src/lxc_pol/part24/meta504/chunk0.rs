//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1511/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1511(t2661: f64, t2662: f64, t4352: f64, t6017: f64, t23285: f64, t2741: f64, t23289: f64, t6035: f64, t61625: f64, t23342: f64, t2652: f64, t221: f64, t23114: f64, t2674: f64, t40683: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76764 = t2661 * t2662 * t4352 * t6017;
    let t76767 = t2741 * t23285;
    let t76793 = t2741 * t23289;
    let t76797 = t2661 * t2662 * t61625 * t6035;
    let t76804 = t2652 * t23342;
    let t76808 = t2674 * t40683 * t221 * t23114;
    (t76764, t76767, t76793, t76797, t76804, t76808)
}
