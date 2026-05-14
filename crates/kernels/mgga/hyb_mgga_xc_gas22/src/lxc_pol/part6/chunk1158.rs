//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1158/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1158<F: Float>(t2233: F, t3313: F, t1333: F, t6561: F, t787: F, t8723: F, t260: F, t8753: F, t6641: F, t2250: F, t3363: F, t1351: F, t6682: F, t2289: F, t3396: F, t1363: F, t6640: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24709 = t3313 * t2233;
    let t24712 = t1333 * t6561;
    let t24774 = t8723 * t787;
    let t24788 = t260 * t8753;
    let t24799 = t260 * t6641;
    let t24813 = t3363 * t2250;
    let t24816 = t1351 * t6682;
    let t24819 = t3396 * t2289;
    let t24822 = t1363 * t6640;
    (t24709, t24712, t24774, t24788, t24799, t24813, t24816, t24819, t24822)
}
