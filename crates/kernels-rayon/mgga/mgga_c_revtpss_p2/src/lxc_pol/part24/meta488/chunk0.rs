//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1481/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1481(t22212: f64, t2516: f64, t6922: f64, t9593: f64, t22185: f64, t2619: f64, t22404: f64, t3920: f64, t13725: f64, t1904: f64, t2439: f64, t22446: f64, t2435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73481 = t22212 * t2516;
    let t73499 = t6922 * t9593;
    let t73515 = t22185 * t2619;
    let t73587 = t22404 * t3920;
    let t73593 = t2439 * t13725 * t1904;
    let t73623 = t2435 * t22446;
    (t73481, t73499, t73515, t73587, t73593, t73623)
}
