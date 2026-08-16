//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta56 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk384;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk385;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk386;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk387;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk388;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta56<F: Float>(t1086: F, t154: F, t486: F, t636: F, t607: F, t123: F, t423: F, t419: F, t409: F, t410: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1087, t1088) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk384::<F>(t1086, t154, t486);
        let t1089 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk385::<F>(t636);
        let t1090 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk386::<F>(t1089, t607);
        let (t1091, t1092, t1094, t1096, t1097, t1098) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk387::<F>(t1088, t1090, t123, t1087, t423, t419);
        let t1099 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk388::<F>(t1098, t409);
        let t1100 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk389::<F>(t410);
    (t1087, t1088, t1089, t1090, t1091, t1092, t1094, t1096, t1097, t1098, t1099, t1100)
}
