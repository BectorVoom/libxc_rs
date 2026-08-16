//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta153 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk973;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk974;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk975;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk976;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta153(t3611: f64, t3612: f64, t1215: f64, t1235: f64, t1246: f64, t3493: f64, t491: f64, t1209: f64, t3032: f64, t3499: f64, t1932: f64, t475: f64, t3590: f64, t493: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t3565: f64, t3604: f64, t3610: f64, t470: f64, t494: f64, t1241: f64, t1238: f64, t1252: f64, t3482: f64, t3484: f64, t3487: f64, t3591: f64, t3593: f64, t3600: f64, t498: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3613, t3617, t3620, t3621, t3623) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk973(t3611, t3612, t1215, t1235, t1246, t3493, t491, t1209, t3032);
        let t3624 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk974(t3499, t3623);
        let (t3625, t3626, t3628, t3630) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk975(t1932, t475, t3611, t3590, t493, t1201, t1244, t1247, t1249, t3565, t3604, t3610, t3613, t3617, t3621, t3624, t470, t494);
        let t3631 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk976(t1241, t3630);
        let t3633 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk977(t1238, t1252, t3482, t3484, t3487, t3591, t3593, t3600, t3631, t498);
    (t3613, t3617, t3620, t3621, t3623, t3624, t3625, t3626, t3628, t3630, t3631, t3633)
}
