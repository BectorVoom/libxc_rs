//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta0 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk0;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk3;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk4;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk5;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk6;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk7;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta0<F: Float>(rho0: F, rho1: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t2 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk0::<F>(rho0, rho1);
        let t3 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1::<F>(rho0, rho1);
        let (t4, t5, t9) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2::<F>(t3, t2);
        let t10 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk3::<F>(t3);
        let (t11, t14) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk4::<F>(t10, t9);
        let t15 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk5::<F>(t10);
        let t16 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk6::<F>(t15);
        let t17 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk7::<F>(t14, t16);
    (t2, t3, t4, t5, t9, t10, t11, t14, t15, t16, t17)
}
