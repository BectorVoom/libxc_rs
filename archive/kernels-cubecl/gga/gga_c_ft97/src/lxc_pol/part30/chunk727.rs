//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 727/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk727<F: Float>(t33307: F, t7515: F, t7511: F, t7512: F, t2506: F, t33283: F, t193: F, t6109: F, t1434: F, t681: F, t7520: F, t2: F, t7440: F) -> (F, F, F, F, F, F, F) {
    let t33308 = t7515 * t33307;
    let t33310 = t7511 * t7512 * t33308;
    let t33312 = t2506 * t33283;
    let t33314 = t6109 * t193 * t33312;
    let t33317 = t1434 * t681 * t7520;
    let t33318 = t33317 / F::cast_from(3.0_f64);
    let t33319 = t2 * t7440;
    (t33308, t33310, t33312, t33314, t33317, t33318, t33319)
}
