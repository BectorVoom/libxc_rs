//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta58 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk403;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk404;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk405;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk406;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta58<F: Float>(t1102: F, t1107: F, t281: F, t415: F, t904: F, t241: F, t457: F, t1090: F, t136: F, t1092: F, t1103: F, t1105: F, t422: F) -> (F, F, F, F, F, F, F, F) {
        let (t1108, t1111) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk403::<F>(t1102, t1107, t281, t415, t904);
        let (t1112, t1113) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk404::<F>(t1111, t241, t457);
        let (t1114, t1115, t1117) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk405::<F>(t1090, t1113, t136, t1092, t1103, t1105, t1108, t1112);
        let t1118 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk406::<F>(t422);
    (t1108, t1111, t1112, t1113, t1114, t1115, t1117, t1118)
}
