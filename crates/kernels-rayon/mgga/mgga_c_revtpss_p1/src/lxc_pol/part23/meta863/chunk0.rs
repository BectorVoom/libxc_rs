//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2754/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2754(t13725: f64, t1904: f64, t2439: f64, t1364: f64, t22441: f64, t786: f64, t22446: f64, t2435: f64, t14079: f64, t14100: f64, t3895: f64, t6919: f64) -> (f64, f64, f64, f64, f64) {
    let t73593 = t2439 * t13725 * t1904;
    let t73598 = t786 * t22441 * t1364;
    let t73623 = t2435 * t22446;
    let t73627 = t14100 * t14079;
    let t73641 = t2439 * t3895 * t6919;
    (t73593, t73598, t73623, t73627, t73641)
}
