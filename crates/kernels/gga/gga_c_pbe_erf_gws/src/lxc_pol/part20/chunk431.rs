//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 431/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk431<F: Float>(t1672: F, t198: F, t185: F, t579: F, t583: F, t582: F, t662: F, t211: F, t174: F, t205: F, t332: F, t395: F, t628: F) -> (F, F, F, F, F, F, F, F) {
    let t1673 = t1672 * t198;
    let t1675 = F::new(4.0) / F::new(135.0) * t185 * t1673;
    let t1676 = t579 * t583;
    let t1683 = t582 * t662;
    let t1684 = t211 * t1683;
    let t1687 = t174 * t332 * t205;
    let t1688 = F::new(0.47988888888888888889e-1) * t1687;
    let t1689 = t395 * t628;
    (t1673, t1675, t1676, t1683, t1684, t1687, t1688, t1689)
}
