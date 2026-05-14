//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 948/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk948<F: Float>(t3920: F, t7496: F, t2098: F, t2453: F, t3908: F, t2097: F, t25937: F, t7282: F, t10073: F, t1426: F, t786: F, t136: F, t2102: F, t2457: F, t25944: F, t2470: F, t7514: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26238 = 0.13009920719177044025e-1 * t7496 * t3920;
    let t26249 = t2453 * t2098;
    let t26251 = 0.11565819519348392139e-2 * t26249 * t3908;
    let t26260 = t25937 * t2097;
    let t26261 = t7282 * t26260;
    let t26263 = 0.24093411633903331839e-3 * t10073 * t26261;
    let t26264 = t2098 * t1426;
    let t26265 = t786 * t26264;
    let t26276 = t2102 * t136;
    let t26277 = t26276 * t2457;
    let t26279 = 0.17135234354032049604e-2 * t25944 * t26277;
    let t26292 = t7514 * t2470;
    (t26238, t26249, t26251, t26260, t26261, t26263, t26264, t26265, t26276, t26277, t26279, t26292)
}
