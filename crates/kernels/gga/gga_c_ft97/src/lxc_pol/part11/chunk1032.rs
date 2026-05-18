//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1032/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1032<F: Float>(t41534: F, t41536: F, t41448: F, t420: F, t701: F, t41499: F, t41502: F, t41505: F, t41508: F, t41513: F, t41516: F, t41519: F, t41522: F, t41525: F, t41528: F, t41531: F) -> (F, F) {
    let t41537 = t41534 * t41536;
    let t41540 = t701 * t420 * t41537 * t41448;
    let t41542 = -F::new(0.68099848938271604939e-1) * t41499 + F::new(0.34049924469135802468e-1) * t41502 + F::new(0.51074886703703703704e-1) * t41505 - F::new(0.51074886703703703704e-1) * t41508 - t41513 + F::new(0.6384360837962962963e-2) * t41516 + F::new(0.94583123525377229081e-2) * t41519 - F::new(0.85124811172839506172e-2) * t41522 - F::new(0.1134997482304526749e-1) * t41525 + F::new(0.26483274587105624143e-1) * t41528 + F::new(0.85124811172839506172e-2) * t41531 + F::new(0.66208186467764060357e-1) * t41540;
    (t41540, t41542)
}
