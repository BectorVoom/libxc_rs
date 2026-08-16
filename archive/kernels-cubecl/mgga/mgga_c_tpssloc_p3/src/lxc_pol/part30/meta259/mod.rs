//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1178;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1179;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1180;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta259<F: Float>(t362: F, t381: F, t884: F, t6784: F, t1949: F, t986: F, t334: F, t371: F, t38: F, t131: F, t350: F, t1009: F, t344: F, t1014: F, t360: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t6785 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1178::<F>(t362, t381);
        let (t6786, t6787, t6790, t6793, t6794, t6795, t6796, t6797) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1179::<F>(t6785, t884, t6784, t1949, t986, t334, t371, t38, t131, t350);
        let t6799 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1180::<F>(t1009, t344, t1014);
        let t6800 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1181::<F>(t360, t68);
    (t6785, t6786, t6787, t6790, t6793, t6794, t6795, t6796, t6797, t6799, t6800)
}
