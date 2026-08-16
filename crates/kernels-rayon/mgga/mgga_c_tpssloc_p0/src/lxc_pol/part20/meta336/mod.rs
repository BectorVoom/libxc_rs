//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1628;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1629;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1630;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta336(t3508: f64, t6739: f64, t11882: f64, t11624: f64, t3612: f64, t1215: f64, t3590: f64, t1246: f64, t11707: f64, t3609: f64, t3623: f64, t3620: f64, t5079: f64, t10471: f64, t1209: f64, t11712: f64, t475: f64, t11616: f64, t11621: f64, t11625: f64, t11640: f64, t11869: f64, t11872: f64, t11877: f64, t11881: f64, t11884: f64, t11888: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t3565: f64, t3604: f64, t3610: f64, t3613: f64, t3617: f64, t3621: f64, t3624: f64, t3626: f64, t3628: f64, t470: f64, t494: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11889, t11890, t11893, t11897, t11904) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1628(t3508, t6739, t11882, t11624, t3612, t1215, t3590, t1246, t11707, t3609);
        let t11907 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1629(t11707, t3623);
        let (t11910, t11913, t11914) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1630(t3620, t5079, t10471, t1209, t11712);
        let (t11915, t11916, t11918) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1631(t475, t6739, t11882, t11616, t11621, t11625, t11640, t11869, t11872, t11877, t11881, t11884, t11888, t11890, t11893, t11897, t11904, t11907, t11910, t11914, t1201, t1244, t1247, t1249, t3565, t3604, t3610, t3613, t3617, t3621, t3624, t3626, t3628, t470, t494);
    (t11889, t11890, t11893, t11897, t11904, t11907, t11910, t11913, t11914, t11915, t11916, t11918)
}
