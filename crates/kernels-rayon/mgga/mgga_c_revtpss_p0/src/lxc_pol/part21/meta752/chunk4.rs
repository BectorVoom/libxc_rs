//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2634/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2634(t13921: f64, t221: f64, t4018: f64, t4019: f64, t2661: f64, t3924: f64, t3992: f64, t5651: f64, t5608: f64, t1882: f64, t4010: f64, t9956: f64) -> (f64, f64, f64, f64) {
    let t48445 = t4018 * t4019 * t221 * t13921;
    let t48449 = t2661 * t3992 * t5651 * t3924;
    let t48453 = t2661 * t3992 * t5608 * t3924;
    let t48455 = t4010 * t1882;
    let t48458 = t2661 * t3992 * t48455 * t9956;
    (t48445, t48449, t48453, t48458)
}
