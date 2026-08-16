//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 717/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk717(t2911: f64, t8470: f64, t2899: f64, t426: f64, t425: f64, t462: f64, t2886: f64, t458: f64, t8465: f64, t2879: f64, t119: f64, t492: f64) -> (f64, f64, f64, f64, f64) {
    let t8471 = t2911 * t8470;
    let t8473 = t426 * t2899;
    let t8475 = t462 * t425;
    let t8476 = t8475 * t2886;
    let t8478 = t8465 * t458;
    let t8479 = t2879 * t8478;
    let t8482 = t492 * t119;
    (t8471, t8473, t8476, t8479, t8482)
}
