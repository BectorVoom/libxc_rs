//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 773/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk773<F: Float>(t10473: F, t3414: F, t7495: F, t5218: F, t3406: F, t7106: F, t5211: F, t10486: F, t10511: F, t7421: F, t7460: F, t1006: F, t3456: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12595 = F::new(8.0) / F::new(27.0) * t10473;
    let t12596 = t7495 * t3414;
    let t12598 = F::new(16.0) / F::new(15.0) * t5218 * t12596;
    let t12599 = t7106 * t3406;
    let t12601 = F::new(16.0) / F::new(15.0) * t5211 * t12599;
    let t12602 = F::new(8.0) / F::new(15.0) * t10486;
    let t12603 = F::new(32.0) / F::new(45.0) * t10511;
    let t12604 = F::new(4.0) / F::new(45.0) * t7421;
    let t12605 = F::new(8.0) / F::new(135.0) * t7460;
    let t12607 = F::new(4.0) / F::new(5.0) * t1006 * t3456;
    (t12595, t12596, t12598, t12599, t12601, t12602, t12603, t12604, t12605, t12607)
}
