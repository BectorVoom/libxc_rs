//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 965/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk965<F: Float>(t31346: F, t4912: F, t7413: F, t7835: F, t8480: F, t30219: F, t8446: F, t1439: F, t30148: F, t30154: F, t7842: F, t1454: F, t30159: F, t7586: F, t1460: F, t355: F, t3706: F) -> (F, F, F, F, F, F) {
    let t35564 = t31346 * t4912;
    let t35567 = t7413 * t8480 * t7835;
    let t35569 = t30219 * t8446;
    let t35573 = t30154 * t7842 * t30148 * t1439;
    let t35580 = t30159 * t7586 * t30148 * t1454;
    let t35585 = t30159 * t7842 * t3706 * t355 * t1460;
    (t35564, t35567, t35569, t35573, t35580, t35585)
}
