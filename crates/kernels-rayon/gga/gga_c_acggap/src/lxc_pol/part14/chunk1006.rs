//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1006/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1006(t35549: f64, t15386: f64, t31057: f64, t35284: f64, t13287: f64, t2302: f64, t4210: f64, t2260: f64, t7852: f64, t30219: f64, t8446: f64, t1439: f64, t30148: f64, t30154: f64, t7842: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35550 = 0.62896184579208304136e-3_f64 * t35549;
    let t35552 = t31057 * t15386 * t35284;
    let t35553 = 0.94344276868812456204e-3_f64 * t35552;
    let t35556 = t31057 * t13287 * t2302 * t4210;
    let t35557 = 0.62896184579208304136e-3_f64 * t35556;
    let t35560 = t7852 * t2260;
    let t35569 = t30219 * t8446;
    let t35570 = 0.31448092289604152068e-2_f64 * t35569;
    let t35573 = t30154 * t7842 * t30148 * t1439;
    (t35550, t35553, t35557, t35560, t35570, t35573)
}
