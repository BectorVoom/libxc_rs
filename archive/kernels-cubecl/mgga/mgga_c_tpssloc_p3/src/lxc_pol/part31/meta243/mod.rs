//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta243 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1022;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1023;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1024;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1025;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1026;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1027;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta243<F: Float>(t6530: F, t666: F, t25: F, t776: F, t154: F, t781: F, t1879: F, t1883: F, t131: F, t209: F, t229: F, t1878: F, t214: F, t252: F, t225: F, t258: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6531, t6542, t6546) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1022::<F>(t6530, t666, t25, t776, t154, t781);
        let t6547 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1023::<F>(t1879, t6546);
        let (t6548, t6551, t6552) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1024::<F>(t1883, t6547, t131, t209, t229, t1878);
        let t6553 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1025::<F>(t214, t252);
        let t6554 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1026::<F>(t225, t258);
        let t6555 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1027::<F>(t6554, t776);
    (t6531, t6542, t6546, t6547, t6548, t6551, t6552, t6553, t6554, t6555)
}
