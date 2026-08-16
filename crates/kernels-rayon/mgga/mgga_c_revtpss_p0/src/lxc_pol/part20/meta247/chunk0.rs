//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1074/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1074(t11202: f64, t996: f64, t3325: f64, t999: f64, t1079: f64, t3043: f64, t378: f64, t3042: f64, t993: f64) -> (f64, f64, f64, f64) {
    let t11203 = t996 * t11202;
    let t11206 = t999 * t3325;
    let t11207 = t1079 * t11206;
    let t11210 = t3043 * t378;
    let t11213 = t3042 * t993;
    (t11203, t11207, t11210, t11213)
}
