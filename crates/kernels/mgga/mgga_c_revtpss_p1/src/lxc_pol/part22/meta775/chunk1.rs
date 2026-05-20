//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2864/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2864<F: Float>(t1122: F, t1261: F, t247: F, t44701: F, t11262: F, t3711: F, t3713: F, t12657: F, t1284: F, t3624: F, t221: F, t461: F, t462: F, t624: F) -> (F, F, F, F) {
    let t44704 = t1261 * t247 * t44701 * t1122;
    let t44751 = t3711 * t11262 * t3713;
    let t44769 = t12657 * t1284 * t3624;
    let t44797 = F::new(5.0) / F::new(486.0) * t461 * t221 * t624 * t462;
    (t44704, t44751, t44769, t44797)
}
