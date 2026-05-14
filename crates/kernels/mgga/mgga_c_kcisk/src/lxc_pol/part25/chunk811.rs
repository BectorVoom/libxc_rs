//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 811/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk811<F: Float>(t5014: F, t5030: F, t1691: F, t604: F, t4825: F, t667: F, t1692: F, t4794: F) -> (F, F, F, F, F, F) {
    let t11179 = t5014 * t5030;
    let t11195 = t1691 * t1691;
    let t11196 = 1.0 / t11195;
    let t11197 = t604 * t11196;
    let t11200 = 1.0 / t4825 / t667;
    let t11204 = t4794 * t1692;
    (t11179, t11195, t11196, t11197, t11200, t11204)
}
