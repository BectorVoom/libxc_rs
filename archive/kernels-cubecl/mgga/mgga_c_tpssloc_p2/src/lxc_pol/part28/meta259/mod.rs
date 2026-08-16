//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1123;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1124;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1125;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1126;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta259<F: Float>(t3: F, t7222: F, t112: F, t2098: F, t2039: F, t671: F, t1401: F, t3938: F, t3941: F, t577: F, t7056: F, t1184: F, t460: F, t33: F, t3953: F, t1437: F, t79: F, t72: F, t1410: F, t605: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t7223, t7230) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1123::<F>(t3, t7222, t112, t2098);
        let (t7235, t7240, t7319, t7428) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1124::<F>(t2039, t671, t1401, t3938, t3941, t577, t7056, t7222, t7230, t1184, t460, t33, t3953);
        let (t7431, t7432) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1125::<F>(t1437, t79, t72);
        let t7435 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1126::<F>(t1410, t605);
    (t7223, t7230, t7235, t7240, t7319, t7428, t7431, t7432, t7435)
}
