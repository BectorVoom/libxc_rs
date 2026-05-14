//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1334/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1334<F: Float>(t1089: F, t11611: F, t458: F, t2709: F, t4510: F, t23904: F, t23910: F, t23924: F, t23932: F, t24084: F, t24087: F, t24088: F, t24090: F, t24095: F, t24097: F, t24099: F, t24104: F, t24108: F, t24109: F, t24113: F, t32502: F, t489: F) -> (F,) {
    let t32574 = t458 * t11611 * t1089;
    let t32576 = t2709 * t4510;
    let t32578 = -240.0 * t24084 - t24087 - 160.0 * t24088 + 48.0 * t24090 + 96.0 * t24095 + 192.0 * t24097 + 0.19751673498613801407e-1 * t32502 * t489 + 64.0 * t24099 + t23904 - t23910 + t23924 + t23932 + 0.10843581300301739842e-1 * t24104 + t24108 + 0.32530743900905219526e-1 * t24109 + t24113 + 2.0 * t32574 + 20.0 * t32576;
    (t32578,)
}
