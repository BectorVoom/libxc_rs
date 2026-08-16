//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1631/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1631<F: Float>(t2970: F, t5828: F, t973: F, t16558: F, t978: F, t977: F, t343: F, t5836: F, t984: F, t4546: F, t10231: F, t5817: F) -> (F, F, F, F, F, F, F) {
    let t17769 = t2970 * t5828;
    let t17770 = t973 * t17769;
    let t17772 = t978 * t16558;
    let t17773 = t977 * t17772;
    let t17777 = t5836 * t984 * t343;
    let t17778 = t4546 * t17777;
    let t17783 = t10231 * t5817;
    (t17769, t17770, t17772, t17773, t17777, t17778, t17783)
}
