//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta375 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1394;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1395;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1396;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1397;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1398;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1399;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta375<F: Float>(t11155: F, t690: F, t11164: F, t11173: F, t2296: F, t3241: F, t39097: F, t11145: F, t123: F, t11147: F, t3240: F, t11153: F, t1088: F, t1089: F, t39110: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t43784 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1394::<F>(t11155, t690);
        let t43786 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1395::<F>(t11164, t690);
        let t43788 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1396::<F>(t11173, t690);
        let (t43791, t43792, t43794) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1397::<F>(t2296, t3241, t39097, t11145, t123);
        let (t43796, t43798) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1398::<F>(t11147, t39097, t123, t3240);
        let (t43800, t43802) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1399::<F>(t11153, t39097, t1088, t123);
        let (t43804, t43806) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1400::<F>(t1089, t39110, t1088, t123);
    (t43784, t43786, t43788, t43791, t43792, t43794, t43796, t43798, t43800, t43802, t43804, t43806)
}
