//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 441/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk441<F: Float>(t1672: F, t198: F, t185: F, t579: F, t583: F, t562: F, t181: F, t184: F, t199: F, t582: F, t662: F, t211: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1673 = t1672 * t198;
    let t1675 = F::new(4.0) / F::new(135.0) * t185 * t1673;
    let t1676 = t579 * t583;
    let t1677 = F::new(8.0) / F::new(45.0) * t1676;
    let t1678 = t562 * t562;
    let t1679 = t1678 * t181;
    let t1680 = t1679 * t184;
    let t1682 = F::new(4.0) / F::new(15.0) * t1680 * t199;
    let t1683 = t582 * t662;
    let t1684 = t211 * t1683;
    (t1673, t1675, t1676, t1677, t1678, t1679, t1680, t1682, t1683, t1684)
}
