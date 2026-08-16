//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1901;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1902;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1903;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta524<F: Float>(t1268: F, t26135: F, t12725: F, t1874: F, t510: F, t652: F, t7000: F, t7685: F, t6876: F, t7688: F, t6999: F, t7753: F, t1983: F, t6880: F, t7754: F, t1982: F, t8944: F, t12461: F, t2018: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26137, t26141, t26142, t26144, t26145, t26147, t26149) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1901::<F>(t1268, t26135, t12725, t1874, t510, t652, t7000, t7685, t6876, t7688, t6999, t7753);
        let (t26150, t26153, t26157, t26161) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1902::<F>(t1983, t26149, t6880, t7685, t6876, t7754, t1982, t8944);
        let t26162 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1903::<F>(t12461, t2018);
    (t26137, t26141, t26142, t26144, t26145, t26147, t26149, t26150, t26153, t26157, t26161, t26162)
}
