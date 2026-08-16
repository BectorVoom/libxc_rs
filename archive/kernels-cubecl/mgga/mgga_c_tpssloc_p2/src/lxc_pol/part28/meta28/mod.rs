//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta28 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk198;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk199;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk200;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk201;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk202;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk203;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk204;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta28<F: Float>(t131: F, t534: F, t221: F, t225: F, t539: F, t144: F, t523: F, t525: F, t533: F, t68: F, t236: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t540, t541, t544) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk198::<F>(t131, t534, t221, t225, t539);
        let t546 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk199::<F>(t144, t225, t523, t525);
        let t547 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk200::<F>(t533);
        let t548 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk201::<F>(t547, t68);
        let t550 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk202::<F>(t546, t548);
        let (t551, t552) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk203::<F>(t550);
        let t553 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk204::<F>(t552, t68);
        let t554 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk205::<F>(t236, t553);
    (t540, t541, t544, t546, t547, t548, t550, t551, t552, t553, t554)
}
