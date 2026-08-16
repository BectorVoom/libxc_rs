//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 565/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk565(t745: f64, t963: f64, t1650: f64, t1662: f64, t1667: f64, t1671: f64, t1674: f64, t1688: f64, t1695: f64, t1699: f64, t1702: f64, t1709: f64, t1710: f64, t1723: f64, t236: f64, t2738: f64, t2741: f64, t2744: f64, t2747: f64) -> f64 {
    let t2750 = t963 * t745;
    let t2753 = t1650 + 0.17315859105681463759e2_f64 * t2738 - t1662 + t1667 - t1671 + 0.84681398666666666666e-3_f64 * t1674 + t1688 - 0.11696447245269292414e1_f64 * t2741 + 0.1350520664e0_f64 * t2744 - t1695 + 0.5848223622634646207e0_f64 * t1699 + 0.5848223622634646207e0_f64 * t2747 * t236 + 0.5848223622634646207e0_f64 * t2750 + t1702 + t1709 + 0.65061487801810439052e-1_f64 * t1710 + t1723;
    t2753
}
