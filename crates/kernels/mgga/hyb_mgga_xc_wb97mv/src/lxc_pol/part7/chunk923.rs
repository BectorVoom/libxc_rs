//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 923/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk923<F: Float>(t2037: F, t2223: F, t191: F, t214: F, t3: F, t674: F, t240: F) -> (F, F, F, F, F) {
    let t8535 = t2223 * t2037;
    let t8536 = t8535 * t191;
    let t8537 = t214 * t3;
    let t8538 = t8537 * t674;
    let t8542 = t240 * t2037;
    let t8543 = t8542 * t191;
    (t8535, t8536, t8538, t8542, t8543)
}
