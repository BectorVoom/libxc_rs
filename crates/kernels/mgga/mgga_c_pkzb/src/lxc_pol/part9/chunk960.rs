//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 960/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk960<F: Float>(t237: F, t7266: F, t7306: F, t7418: F, t7521: F, t1991: F, t2860: F, t1954: F, t2848: F, t723: F, t730: F, t1107: F, t5498: F) -> (F, F, F, F, F, F) {
    let t7524 = t237 * (t7266 + t7306 + t7418 + t7521);
    let t7526 = F::cast_from(0.11696447245269292414e1_f64) * t2860 * t1991;
    let t7527 = t1954 * t2848;
    let t7528 = t7527 * t723;
    let t7530 = F::cast_from(0.23392894490538584828e1_f64) * t730 * t7528;
    let t7531 = t5498 * t1107;
    (t7524, t7526, t7527, t7528, t7530, t7531)
}
