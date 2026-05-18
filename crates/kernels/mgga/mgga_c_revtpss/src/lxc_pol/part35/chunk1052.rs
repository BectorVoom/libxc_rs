//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1052/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1052<F: Float>(t10073: F, t26261: F, t1426: F, t2098: F, t786: F, t136: F, t2102: F, t2457: F, t25944: F, t2470: F, t7514: F, t7284: F) -> (F, F, F, F, F, F, F, F) {
    let t26263 = F::new(0.24093411633903331839e-3) * t10073 * t26261;
    let t26264 = t2098 * t1426;
    let t26265 = t786 * t26264;
    let t26276 = t2102 * t136;
    let t26277 = t26276 * t2457;
    let t26279 = F::new(0.17135234354032049604e-2) * t25944 * t26277;
    let t26292 = t7514 * t2470;
    let t26294 = F::new(0.96373646535613327357e-2) * t7284 * t26292;
    (t26263, t26264, t26265, t26276, t26277, t26279, t26292, t26294)
}
