//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 721/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk721<F: Float>(t3406: F, t7106: F, t5211: F, t10486: F, t10511: F, t7421: F, t7460: F, t1006: F, t3456: F, t10617: F, t12582: F, t12587: F, t12592: F, t12593: F, t12595: F, t12598: F, t5906: F, t5919: F, t5922: F, t8425: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12599 = t7106 * t3406;
    let t12601 = 16.0 / 15.0 * t5211 * t12599;
    let t12602 = 8.0 / 15.0 * t10486;
    let t12603 = 32.0 / 45.0 * t10511;
    let t12604 = 4.0 / 45.0 * t7421;
    let t12605 = 8.0 / 135.0 * t7460;
    let t12607 = 4.0 / 5.0 * t1006 * t3456;
    let t12608 = 8.0 / 15.0 * t10617;
    let t12609 = -t12582 - t12587 + t5906 - t12592 + t12593 + 0.33545228223331014468e-1 * t8425 - t5919 + t5922 + t12595 - t12598 - t12601 + t12602 + t12603 - t12604 - t12605 + t12607 - t12608;
    (t12599, t12601, t12602, t12603, t12604, t12605, t12607, t12608, t12609)
}
