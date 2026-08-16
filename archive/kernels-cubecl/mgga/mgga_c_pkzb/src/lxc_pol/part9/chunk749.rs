//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 749/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk749<F: Float>(t1736: F, t5257: F, t1692: F, t581: F, t582: F, t1698: F, t579: F, t583: F, t1702: F, t1712: F, t50: F, t5217: F) -> (F, F, F, F, F, F) {
    let t5258 = t5257 * t1736;
    let t5261 = t581 * t582 * t1692;
    let t5264 = t1698 * t579;
    let t5265 = t5264 * t583;
    let t5267 = t1702 * t1712;
    let t5269 = t50 * t5217;
    (t5258, t5261, t5264, t5265, t5267, t5269)
}
