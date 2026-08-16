//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1146/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1146(t15386: f64, t31195: f64, t36323: f64, t30248: f64, t542: f64, t1967: f64, t8855: f64, t31773: f64, t8916: f64, t7447: f64, t8920: f64, t2001: f64, t4355: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36347 = t31195 * t15386 * t36323;
    let t36349 = t30248 * t542;
    let t36351 = t1967 * t8855;
    let t36353 = t31773 * t8916;
    let t36355 = t7447 * t8920;
    let t36358 = t2001 * t4355;
    (t36347, t36349, t36351, t36353, t36355, t36358)
}
