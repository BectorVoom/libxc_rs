//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 668/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk668<F: Float>(t10842: F, t11188: F, t1791: F, t1691: F, t604: F, t1790: F, t4824: F, t4825: F, t667: F, t1692: F, t4794: F, t10471: F, t140: F, t673: F, t1896: F, t1901: F) -> (F, F, F, F, F, F) {
    let t11189 = t10842 + t11188;
    let t11190 = t11189 * t1791;
    let t11195 = t1691 * t1691;
    let t11196 = 1.0 / t11195;
    let t11197 = t604 * t11196;
    let t11198 = t4824 * t1790;
    let t11200 = 1.0 / t4825 / t667;
    let t11201 = t11198 * t11200;
    let t11204 = t4794 * t1692;
    let t11208 = t140 * t10471 * t673;
    let t11209 = t11208 * t1896;
    let t11211 = t11208 * t1901;
    (t11190, t11197, t11201, t11204, t11209, t11211)
}
