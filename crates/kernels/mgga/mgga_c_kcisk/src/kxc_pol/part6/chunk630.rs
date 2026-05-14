//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 630/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk630<F: Float>(t5014: F, t5030: F, t1691: F, t604: F, t4825: F, t667: F, t10471: F, t140: F, t673: F, t4971: F, t654: F, t4597: F, t642: F, t1870: F, t704: F, t139: F, t5911: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11179 = t5014 * t5030;
    let t11195 = t1691 * t1691;
    let t11196 = 1.0 / t11195;
    let t11197 = t604 * t11196;
    let t11200 = 1.0 / t4825 / t667;
    let t11208 = t140 * t10471 * t673;
    let t11213 = t654 * t4971;
    let t11218 = t642 * t4597;
    let t11224 = t1870 * t1870;
    let t11225 = 1.0 / t11224;
    let t11226 = t704 * t11225;
    let t11227 = t11226 * sigma2;
    let t11250 = t139 * t5911;
    (t11179, t11197, t11200, t11208, t11213, t11218, t11225, t11226, t11227, t11250)
}
