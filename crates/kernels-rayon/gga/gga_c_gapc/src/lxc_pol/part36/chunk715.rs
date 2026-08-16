//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 715/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk715(t2886: f64, t8482: f64, t118: f64, t1845: f64, t61: f64, t2881: f64, t2921: f64, t8350: f64, t2925: f64, t1457: f64, t424: f64, t2920: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8483 = t8482 * t2886;
    let t8485 = t1845 * t118;
    let t8486 = t61 * t8485;
    let t8487 = t8486 * t2881;
    let t8489 = t8350 * t2921;
    let t8490 = t8489 * t2925;
    let t8492 = t424 * t1457;
    let t8493 = t2920 * t8492;
    (t8483, t8487, t8489, t8490, t8492, t8493)
}
