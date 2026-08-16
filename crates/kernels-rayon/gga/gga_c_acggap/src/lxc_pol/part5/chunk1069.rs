//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1069/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1069(t3077: f64, t4189: f64, t1160: f64, t1603: f64, t322: f64, t407: f64, t1410: f64, t441: f64, t1633: f64, t17386: f64, t1539: f64, t545: f64, t943: f64) -> (f64, f64, f64, f64, f64) {
    let t18953 = t3077 * t4189;
    let t18957 = t1160 * t1603 * t322 * t407;
    let t18973 = t441 * t1410;
    let t18977 = t17386 * t1633;
    let t18989 = t1160 * t545 * t943 * t1539;
    (t18953, t18957, t18973, t18977, t18989)
}
