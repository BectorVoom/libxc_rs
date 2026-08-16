//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 851/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk851(t16409: f64, t16442: f64, t1299: f64, t4675: f64, t3462: f64, t4723: f64, t116: f64, t16221: f64, t6944: f64, t16329: f64, t696: f64, t16325: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16443 = t16409 + t16442;
    let t16456 = t4675 * t1299;
    let t16460 = t3462 * t4723;
    let t16464 = t6944 * t116 * t16221;
    let t16471 = t696 * t16329;
    let t16474 = t696 * t16325;
    (t16443, t16456, t16460, t16464, t16471, t16474)
}
