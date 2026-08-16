//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2222/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2222(t1204: f64, t8190: f64, t2142: f64, t5284: f64, t3153: f64, t1276: f64, t42859: f64, t13038: f64, t2149: f64, t11249: f64, t29157: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104465 = t1204 * t8190;
    let t104472 = t2142 * t5284;
    let t104473 = t104472 * t3153;
    let t104480 = t42859 * t1276;
    let t104482 = t2149 * t104480 * t13038;
    let t104483 = t29157 * t11249;
    let t104490 = t104472 * t73;
    (t104465, t104473, t104480, t104482, t104483, t104490)
}
