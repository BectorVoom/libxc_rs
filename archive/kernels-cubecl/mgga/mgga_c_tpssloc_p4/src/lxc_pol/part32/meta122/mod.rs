//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta122 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk712;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk713;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk714;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk715;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk716;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta122<F: Float>(t3031: F, t3032: F, t371: F, t335: F, t368: F, t1015: F, t1030: F, t372: F, t364: F, t354: F, t1043: F, t121: F, t248: F, t884: F, t1041: F, t283: F, t883: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3033, t3034) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk712::<F>(t3031, t3032, t371);
        let t3036 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk713::<F>(t3034, t335);
        let (t3037, t3038, t3039) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk714::<F>(t3036, t368, t1015, t3033);
        let (t3047, t3048, t3051) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk715::<F>(t1030, t372, t364, t354, t1043, t121);
        let (t3053, t3054, t3061) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk716::<F>(t248, t3051, t884, t1041, t283, t883);
    (t3033, t3034, t3036, t3037, t3038, t3039, t3047, t3048, t3051, t3053, t3054, t3061)
}
