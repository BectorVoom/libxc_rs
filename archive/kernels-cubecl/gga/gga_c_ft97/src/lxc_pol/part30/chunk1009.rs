//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1009/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1009<F: Float>(t24437: F, t2574: F, t27814: F, t33341: F, t1434: F, t150133: F, t193: F, t2506: F, t33282: F, t33288: F, t35310: F, t3938: F, t7440: F) -> (F, F, F, F) {
    let t150220 = t24437 * t2574 * t33341 * t27814;
    let t150224 = t1434 * t193 * t2506 * t150133;
    let t150227 = t33282 * t33288 * t35310;
    let t150228 = t7440 * t3938;
    (t150220, t150224, t150227, t150228)
}
