//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta177 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk930;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk931;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk932;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk933;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk934;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk935;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta177(t3862: f64, t555: f64, t1361: f64, t835: f64, t1336: f64, t1369: f64, t1995: f64, t241: f64, t67: f64, t3734: f64, t820: f64, t1367: f64, t3719: f64, t1315: f64, t1341: f64, t1354: f64, t1363: f64, t3733: f64, t3762: f64, t3763: f64, t3766: f64, t3770: f64, t3774: f64, t3778: f64, t3781: f64, t3783: f64, t3790: f64, t3795: f64, t3800: f64, t3803: f64, t3809: f64, t3853: f64, t3858: f64, t559: f64, t539: f64, t1373: f64, t225: f64, t1376: f64, t566: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3864, t3865, t3866) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk930(t3862, t555, t1361, t835, t1336);
        let (t3867, t3870, t3872) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk931(t1369, t3866, t1995, t241, t67, t3734, t820);
        let t3876 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk932(t1367, t3719, t820);
        let t3879 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk933(t1315, t1341, t1354, t1363, t1369, t3733, t3762, t3763, t3766, t3770, t3774, t3778, t3781, t3783, t3790, t3795, t3800, t3803, t3809, t3853, t3858, t3864, t3867, t3872, t3876, t559);
        let (t3880, t3882) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk934(t3879, t539, t1373, t225);
        let t3886 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk935(t1376, t566);
        let t3887 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk936(t3886, t68);
    (t3864, t3865, t3866, t3867, t3870, t3872, t3876, t3879, t3880, t3882, t3886, t3887)
}
