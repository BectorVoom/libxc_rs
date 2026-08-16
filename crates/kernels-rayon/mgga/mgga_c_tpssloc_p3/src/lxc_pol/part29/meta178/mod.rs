//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk937;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk938;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk939;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk940;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk941;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta178(t1385: f64, t3887: f64, t3787: f64, t562: f64, t3793: f64, t1338: f64, t1372: f64, t1352: f64, t1380: f64, t3851: f64, t3856: f64, t3879: f64, t553: f64, t1332: f64, t1336: f64, t1381: f64, t1383: f64, t3773: f64, t3777: f64, t544: f64, t564: f64, t1378: f64, t1375: f64, t1386: f64, t3753: f64, t3755: f64, t3758: f64, t3880: f64, t3882: f64, t568: f64, t193: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3888 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk937(t1385);
        let t3889 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk938(t3887, t3888);
        let (t3898, t3901, t3902, t3905, t3907, t3909, t3911) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk939(t3787, t562, t3793, t1338, t1372, t1352, t1380, t3851, t3856, t3879, t553, t1332, t1336, t1381, t1383, t3773, t3777, t544, t564);
        let t3912 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk940(t1378, t3911);
        let t3914 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk941(t1375, t1386, t3753, t3755, t3758, t3880, t3882, t3889, t3912, t568);
        let t3918 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk942(t193, t532);
    (t3888, t3889, t3898, t3901, t3902, t3905, t3907, t3909, t3911, t3912, t3914, t3918)
}
