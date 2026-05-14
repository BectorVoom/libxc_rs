//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 545/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk545<F: Float>(t745: F, t963: F, t1650: F, t1662: F, t1667: F, t1671: F, t1674: F, t1688: F, t1695: F, t1699: F, t1702: F, t1709: F, t1710: F, t1723: F, t236: F, t2738: F, t2741: F, t2744: F, t2747: F) -> (F,) {
    let t2750 = t963 * t745;
    let t2753 = t1650 + 0.17315859105681463759e2 * t2738 - t1662 + t1667 - t1671 + 0.84681398666666666666e-3 * t1674 + t1688 - 0.11696447245269292414e1 * t2741 + 0.1350520664e0 * t2744 - t1695 + 0.5848223622634646207e0 * t1699 + 0.5848223622634646207e0 * t2747 * t236 + 0.5848223622634646207e0 * t2750 + t1702 + t1709 + 0.65061487801810439052e-1 * t1710 + t1723;
    (t2753,)
}
