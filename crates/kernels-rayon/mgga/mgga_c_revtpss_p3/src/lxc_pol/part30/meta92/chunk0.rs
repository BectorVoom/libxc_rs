//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 586/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk586(t1963: f64, t30: f64, t1940: f64, t1962: f64, t207: f64, t198: f64, t892: f64, t33: f64, t1312: f64, t1936: f64, t196: f64, t511: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1964 = t1963 * t30;
    let t1966 = t1940 * t1964 / 2.0_f64;
    let t1993 = t207 * t1962;
    let t1995 = t198 * t1993 * t892;
    let t2000 = t1963 * t33;
    let t2002 = t1940 * t2000 / 2.0_f64;
    let t2010 = 2.0_f64 * t1312 * t1936;
    let t2013 = t511 * t196;
    (t1966, t1993, t1995, t2002, t2010, t2013)
}
