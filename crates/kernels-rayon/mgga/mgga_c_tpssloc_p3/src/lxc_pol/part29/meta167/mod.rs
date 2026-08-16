//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk888;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk889;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk890;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk891;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk892;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk893;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta167(t1932: f64, t475: f64, t3611: f64, t3590: f64, t493: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t3565: f64, t3604: f64, t3610: f64, t3613: f64, t3617: f64, t3621: f64, t3624: f64, t470: f64, t494: f64, t1241: f64, t1238: f64, t1252: f64, t3482: f64, t3484: f64, t3487: f64, t3591: f64, t3593: f64, t3600: f64, t498: f64, t1254: f64, t500: f64, t1256: f64, t193: f64, t3258: f64, t3261: f64, t3268: f64, t3310: f64, t3318: f64, t336: f64, t3408: f64, t3410: f64, t3413: f64, t3417: f64, t3421: f64, t3425: f64, t28: f64, t265: f64, t504: f64, t2756: f64, t1081: f64, t1260: f64, t2250: f64, t3231: f64, t506: f64, t52: f64, t607: f64, t873: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t3227: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3625, t3626, t3628, t3630) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk888(t1932, t475, t3611, t3590, t493, t1201, t1244, t1247, t1249, t3565, t3604, t3610, t3613, t3617, t3621, t3624, t470, t494);
        let t3631 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk889(t1241, t3630);
        let (t3633, t3637, t3639) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk890(t1238, t1252, t3482, t3484, t3487, t3591, t3593, t3600, t3631, t498, t1254, t500);
        let t3640 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk891(t3639);
        let t3643 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk892(t1256, t193, t3258, t3261, t3268, t3310, t3318, t336, t3408, t3410, t3413, t3417, t3421, t3425, t3633, t3637, t3640);
        let (t3644, t3651) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk893(t28, t265, t504, t2756, t3643, t1081, t1260, t2250, t3231, t506, t52, t607, t873, dens_threshold, rho1, zeta_threshold);
        let t3652 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk894(t3227, t3651);
    (t3625, t3626, t3628, t3630, t3631, t3633, t3637, t3639, t3640, t3644, t3652)
}
