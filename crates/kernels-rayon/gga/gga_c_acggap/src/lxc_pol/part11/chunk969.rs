//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 969/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk969(t29984: f64, t315: f64, t2134: f64, t1960: f64, t3883: f64, t119: f64, t7877: f64, t3912: f64, t7976: f64, t872: f64, t3919: f64, t7948: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32063 = t315 * t29984;
    let t32064 = t32063 * t2134;
    let t32066 = t1960 * t3883;
    let t32069 = t119 * t7877;
    let t32073 = 0.65854491829355115987e0_f64 * t1960 * t3912;
    let t32080 = t7976 * t872;
    let t32082 = t7948 * t3919;
    (t32064, t32066, t32069, t32073, t32080, t32082)
}
