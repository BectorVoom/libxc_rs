//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 873/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk873(t13846: f64, t1941: f64, t13877: f64, t2018: f64, t5617: f64, t807: f64, t241: f64, t25981: f64, t820: f64, t5677: f64, t26028: f64, t5697: f64) -> (f64, f64, f64, f64) {
    let t27932 = t1941 * t13846;
    let t27933 = t27932 * t13877;
    let t27936 = t2018 * t5617;
    let t27937 = t807 * t27936;
    let t27940 = t820 * t25981 * t241;
    let t27941 = t27940 * t5677;
    let t27943 = t26028 * t5697;
    (t27933, t27937, t27941, t27943)
}
