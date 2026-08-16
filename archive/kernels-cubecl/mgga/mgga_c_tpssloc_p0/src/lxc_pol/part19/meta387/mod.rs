//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta387 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1455;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1456;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta387<F: Float>(t42339: F, t466: F, t11715: F, t42341: F, t3507: F, t491: F, t11721: F, t23508: F, t1009: F, t11598: F, t1243: F, t3590: F, t11714: F, t476: F, t3508: F, t11883: F, t3493: F, t11889: F, t11620: F, t11638: F, t11639: F, t11877: F, t11881: F, t11888: F, t11893: F, t11904: F, t11914: F, t11915: F, t1235: F, t1244: F, t1246: F, t1247: F, t3610: F, t3611: F, t3617: F, t3624: F, t3625: F, t44673: F, t5068: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44696, t44698, t44699, t44700, t44701, t44706, t44707, t44710) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1455::<F>(t42339, t466, t11715, t42341, t3507, t491, t11721, t23508, t1009, t11598, t1243, t3590);
        let (t44722, t44725, t44748) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1456::<F>(t11714, t476, t42341, t44696, t3508, t23508, t11883, t3493, t11889, t11620, t11638, t11639, t11877, t11881, t11888, t11893, t11904, t11914, t11915, t1235, t1244, t1246, t1247, t3610, t3611, t3617, t3624, t3625, t44673, t44700, t44707, t44710, t5068);
    (t44696, t44698, t44699, t44700, t44701, t44706, t44710, t44722, t44725, t44748)
}
