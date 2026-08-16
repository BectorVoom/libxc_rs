//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1628;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1629;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1630;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta336<F: Float>(t3508: F, t6739: F, t11882: F, t11624: F, t3612: F, t1215: F, t3590: F, t1246: F, t11707: F, t3609: F, t3623: F, t3620: F, t5079: F, t10471: F, t1209: F, t11712: F, t475: F, t11616: F, t11621: F, t11625: F, t11640: F, t11869: F, t11872: F, t11877: F, t11881: F, t11884: F, t11888: F, t1201: F, t1244: F, t1247: F, t1249: F, t3565: F, t3604: F, t3610: F, t3613: F, t3617: F, t3621: F, t3624: F, t3626: F, t3628: F, t470: F, t494: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11889, t11890, t11893, t11897, t11904) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1628::<F>(t3508, t6739, t11882, t11624, t3612, t1215, t3590, t1246, t11707, t3609);
        let t11907 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1629::<F>(t11707, t3623);
        let (t11910, t11913, t11914) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1630::<F>(t3620, t5079, t10471, t1209, t11712);
        let (t11915, t11916, t11918) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1631::<F>(t475, t6739, t11882, t11616, t11621, t11625, t11640, t11869, t11872, t11877, t11881, t11884, t11888, t11890, t11893, t11897, t11904, t11907, t11910, t11914, t1201, t1244, t1247, t1249, t3565, t3604, t3610, t3613, t3617, t3621, t3624, t3626, t3628, t470, t494);
    (t11889, t11890, t11893, t11897, t11904, t11907, t11910, t11913, t11914, t11915, t11916, t11918)
}
