//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 572/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk572<F: Float>(t1167: F, t931: F, t824: F, t2888: F, t154: F, t3026: F, t907: F, t178: F, t2365: F, t2364: F) -> (F, F, F, F, F, F) {
    let t3175 = t931 * t1167;
    let t3176 = t3175 * t824;
    let t3177 = t2888 * t3176;
    let t3181 = t154 * t907 * t3026;
    let t3184 = t2365 * t178;
    let t3185 = t2364 * t3184;
    (t3175, t3176, t3177, t3181, t3184, t3185)
}
