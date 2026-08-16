//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta387 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1455;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1456;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta387(t42339: f64, t466: f64, t11715: f64, t42341: f64, t3507: f64, t491: f64, t11721: f64, t23508: f64, t1009: f64, t11598: f64, t1243: f64, t3590: f64, t11714: f64, t476: f64, t3508: f64, t11883: f64, t3493: f64, t11889: f64, t11620: f64, t11638: f64, t11639: f64, t11877: f64, t11881: f64, t11888: f64, t11893: f64, t11904: f64, t11914: f64, t11915: f64, t1235: f64, t1244: f64, t1246: f64, t1247: f64, t3610: f64, t3611: f64, t3617: f64, t3624: f64, t3625: f64, t44673: f64, t5068: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44696, t44698, t44699, t44700, t44701, t44706, t44707, t44710) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1455(t42339, t466, t11715, t42341, t3507, t491, t11721, t23508, t1009, t11598, t1243, t3590);
        let (t44722, t44725, t44748) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1456(t11714, t476, t42341, t44696, t3508, t23508, t11883, t3493, t11889, t11620, t11638, t11639, t11877, t11881, t11888, t11893, t11904, t11914, t11915, t1235, t1244, t1246, t1247, t3610, t3611, t3617, t3624, t3625, t44673, t44700, t44707, t44710, t5068);
    (t44696, t44698, t44699, t44700, t44701, t44706, t44710, t44722, t44725, t44748)
}
