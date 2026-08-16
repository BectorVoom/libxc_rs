//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta153 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk973;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk974;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk975;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk976;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta153<F: Float>(t3611: F, t3612: F, t1215: F, t1235: F, t1246: F, t3493: F, t491: F, t1209: F, t3032: F, t3499: F, t1932: F, t475: F, t3590: F, t493: F, t1201: F, t1244: F, t1247: F, t1249: F, t3565: F, t3604: F, t3610: F, t470: F, t494: F, t1241: F, t1238: F, t1252: F, t3482: F, t3484: F, t3487: F, t3591: F, t3593: F, t3600: F, t498: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3613, t3617, t3620, t3621, t3623) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk973::<F>(t3611, t3612, t1215, t1235, t1246, t3493, t491, t1209, t3032);
        let t3624 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk974::<F>(t3499, t3623);
        let (t3625, t3626, t3628, t3630) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk975::<F>(t1932, t475, t3611, t3590, t493, t1201, t1244, t1247, t1249, t3565, t3604, t3610, t3613, t3617, t3621, t3624, t470, t494);
        let t3631 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk976::<F>(t1241, t3630);
        let t3633 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk977::<F>(t1238, t1252, t3482, t3484, t3487, t3591, t3593, t3600, t3631, t498);
    (t3613, t3617, t3620, t3621, t3623, t3624, t3625, t3626, t3628, t3630, t3631, t3633)
}
