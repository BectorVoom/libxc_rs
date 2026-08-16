//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta162 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk862;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk863;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk864;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk865;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk866;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk867;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta162<F: Float>(t121: F, t1229: F, t1090: F, t248: F, t1227: F, t1230: F, t3252: F, t3248: F, t1009: F, t1190: F, t1011: F, t1212: F, t374: F, t486: F, t677: F, t485: F, t1203: F, t1222: F, t221: F, t3426: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3521 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk862::<F>(t121, t1229);
        let t3523 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk863::<F>(t1090, t248, t3521);
        let (t3524, t3527) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk864::<F>(t1227, t3523, t1230, t248, t3252);
        let t3531 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk865::<F>(t1230, t248, t3248);
        let (t3534, t3535, t3536) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk866::<F>(t1009, t1190, t1011, t1212);
        let t3540 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk867::<F>(t374, t486, t677);
        let (t3542, t3543, t3545) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk868::<F>(t3540, t485, t1203, t1222, t221, t3426);
    (t3521, t3523, t3524, t3527, t3531, t3534, t3535, t3536, t3540, t3542, t3543, t3545)
}
