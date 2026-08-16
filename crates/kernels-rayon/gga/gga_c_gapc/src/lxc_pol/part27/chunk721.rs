//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 721/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk721(t119: f64, t492: f64, t2886: f64, t118: f64, t1845: f64, t61: f64, t2881: f64, t2921: f64, t8350: f64, t2925: f64, t1457: f64, t424: f64) -> (f64, f64, f64, f64, f64) {
    let t8482 = t492 * t119;
    let t8483 = t8482 * t2886;
    let t8485 = t1845 * t118;
    let t8486 = t61 * t8485;
    let t8487 = t8486 * t2881;
    let t8489 = t8350 * t2921;
    let t8490 = t8489 * t2925;
    let t8492 = t424 * t1457;
    (t8483, t8487, t8489, t8490, t8492)
}
