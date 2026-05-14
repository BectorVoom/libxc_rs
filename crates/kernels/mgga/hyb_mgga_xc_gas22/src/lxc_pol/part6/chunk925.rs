//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 925/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk925<F: Float>(t839: F, t848: F, t8709: F, t8651: F, t6528: F, t6530: F, t6533: F, t8648: F, t8676: F, t251: F, t260: F, t3396: F, t1347: F, t2189: F, t2234: F, t1379: F, t2322: F, t2326: F, t2334: F, t3430: F, t3436: F, t3440: F, t6759: F, t856: F, t858: F, t8601: F, t8605: F, t8608: F, t8613: F, t8616: F, t8620: F, t8623: F, t8627: F) -> (F, F, F, F, F, F, F, F) {
    let t8711 = t839 * t8709 * t848;
    let t8721 = 0.35616666666666666666e-1 * t8651;
    let t8723 = -t6528 + 0.47488888888888888888e-1 * t6530 - 0.17808333333333333333e-1 * t6533 + 0.23744444444444444444e-1 * t8676 - t8721 + 0.53425e-1 * t8648;
    let t8725 = 0.621814e-1 * t8723 * t251;
    let t8726 = t260 * t3396;
    let t8731 = t1347 * t2189;
    let t8733 = 6.0 * t2234 * t8731;
    let t8734 = -0.10254018858216406658e4 * t856 * t8601 - 0.34631718211362927518e2 * t856 * t8605 - 0.17315859105681463759e2 * t856 * t8608 - t8613 - t8616 + t8620 + t8623 + t8627 - 0.5848223622634646207e0 * t6759 * t1379 + 0.11696447245269292414e1 * t3430 * t2326 - 0.5848223622634646207e0 * t856 * t8711 - 0.17315859105681463759e2 * t3430 * t2334 + 0.23392894490538584828e1 * t2322 * t3436 - t8725 - 0.11696447245269292414e1 * t8726 * t858 - 0.11696447245269292414e1 * t2322 * t3440 + t8733;
    (t8711, t8721, t8723, t8725, t8726, t8731, t8733, t8734)
}
