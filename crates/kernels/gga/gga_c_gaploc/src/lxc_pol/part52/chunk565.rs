//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 565/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk565<F: Float>(t10646: F, t11578: F, t11589: F, t11592: F, t11596: F, t11599: F, t11636: F, t11672: F, t11697: F, t1897: F, t2508: F, t270: F, t3617: F, t3622: F, t3627: F, t3631: F, t681: F) -> (F,) {
    let t11699 = -0.17090058289204942853e-2 * t10646 - 0.76905262301422242837e-2 * t681 * t3631 - 0.76905262301422242837e-2 * t270 * t11578 - 0.23071578690426672851e-1 * t681 * t3622 + 0.15381052460284448567e-1 * t681 * t3627 + 0.76905262301422242837e-2 * t681 * t3617 + 0.76905262301422242837e-2 * t270 * t11589 - 0.76905262301422242837e-2 * t1897 * t11592 - 0.23071578690426672851e-1 * t2508 * t11596 - 0.53833683610995569986e-1 * t2508 * t11599 + t11636 + t11672 + t11697;
    (t11699,)
}
