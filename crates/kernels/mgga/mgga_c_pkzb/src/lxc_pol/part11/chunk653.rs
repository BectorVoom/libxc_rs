//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 653/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk653<F: Float>(t1197: F, t1209: F, t2257: F, t2279: F, t2296: F, t2318: F, t3083: F, t3116: F, t365: F, t3736: F, t3738: F, t3742: F, t3768: F, t3771: F, t3774: F, t3780: F, t3793: F, t3796: F, t3802: F, t3807: F, t3820: F, t3823: F, t863: F, t882: F) -> (F,) {
    let t3826 = -0.310907e-1 * t3774 * t365 + 2.0 * t3083 * t1197 - 2.0 * t2257 * t3780 + 1.0 * t863 * t3793 + 0.32163958997385070134e2 * t2279 * t3796 + t3736 - t3738 + t3742 - t3768 - t3771 - 0.19751673498613801407e-1 * t3802 + 0.11696447245269292414e1 * t3116 * t1209 - 0.11696447245269292414e1 * t2296 * t3807 + 0.5848223622634646207e0 * t882 * t3820 + 0.17315859105681463759e2 * t2318 * t3823;
    (t3826,)
}
