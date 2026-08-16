//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta68 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk424;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk425;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk426;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk427;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta68<F: Float>(t1395: F, t3: F, t576: F, t112: F, t577: F, t671: F, t71: F, t79: F, t193: F, t202: F, t154: F, t204: F, t119: F, t210: F, t201: F, t243: F, t365: F, t335: F, t371: F, t532: F, t556: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1396, t1398, t1401, t1404, t1864, t1877, t1878) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk424::<F>(t1395, t3, t576, t112, t577, t671, t71, t79, t193, t202, t154, t204);
        let (t1887, t1891) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk425::<F>(t119, t210, t201, t243);
        let (t1929, t1932) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk426::<F>(t365, t335, t371);
        let t1995 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk427::<F>(t532, t556);
    (t1396, t1398, t1401, t1404, t1864, t1877, t1878, t1887, t1891, t1929, t1932, t1995)
}
