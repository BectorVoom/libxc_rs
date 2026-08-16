//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 778/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk778(t12366: f64, t2312: f64, t12427: f64, t484: f64, t20883: f64, t6525: f64, t9079: f64, t2478: f64, t3133: f64, t6583: f64, t30839: f64, t901: f64) -> (f64, f64, f64, f64, f64) {
    let t39899 = t2312 * t12366;
    let t39901 = t484 * t12427;
    let t39904 = t6525 * t9079 * t20883;
    let t39968 = t6583 * t3133 * t2478;
    let t40007 = t30839 * t901;
    (t39899, t39901, t39904, t39968, t40007)
}
