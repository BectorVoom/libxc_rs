//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 903/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk903<F: Float>(t35549: F, t15386: F, t31057: F, t35284: F, t13287: F, t2302: F, t4210: F, t2260: F, t7852: F, t30219: F, t8446: F, t1439: F, t30148: F, t30154: F, t7842: F, t1454: F, t30159: F, t7586: F) -> (F, F, F, F, F, F, F) {
    let t35550 = 0.62896184579208304136e-3 * t35549;
    let t35552 = t31057 * t15386 * t35284;
    let t35553 = 0.94344276868812456204e-3 * t35552;
    let t35556 = t31057 * t13287 * t2302 * t4210;
    let t35557 = 0.62896184579208304136e-3 * t35556;
    let t35560 = t7852 * t2260;
    let t35569 = t30219 * t8446;
    let t35570 = 0.31448092289604152068e-2 * t35569;
    let t35573 = t30154 * t7842 * t30148 * t1439;
    let t35574 = 0.31448092289604152068e-2 * t35573;
    let t35580 = t30159 * t7586 * t30148 * t1454;
    (t35550, t35553, t35557, t35560, t35570, t35574, t35580)
}
