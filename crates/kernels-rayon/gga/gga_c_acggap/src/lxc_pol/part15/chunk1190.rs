//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1190/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1190(t15386: f64, t34823: f64, t40066: f64, t615: f64, t6413: f64, t1907: f64, t618: f64, t2137: f64, t1410: f64, t157: f64, t556: f64, t315: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40587 = t34823 * t15386 * t40066;
    let t40601 = t615 * t6413;
    let t40619 = t1907 * t618;
    let t40620 = t615 * t40619;
    let t40653 = t2137 * t40619;
    let t40675 = t556 * t1410 * t157;
    let t40697 = t315 * t40619;
    (t40587, t40601, t40620, t40653, t40675, t40697)
}
