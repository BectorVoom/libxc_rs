//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk927;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk928;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk929;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk930;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk931;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk932;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta178<F: Float>(t3862: F, t555: F, t1361: F, t835: F, t1336: F, t1369: F, t1995: F, t241: F, t67: F, t3734: F, t820: F, t1367: F, t3719: F, t1315: F, t1341: F, t1354: F, t1363: F, t3733: F, t3762: F, t3763: F, t3766: F, t3770: F, t3774: F, t3778: F, t3781: F, t3783: F, t3790: F, t3795: F, t3800: F, t3803: F, t3809: F, t3853: F, t3858: F, t559: F, t539: F, t1373: F, t225: F, t1376: F, t566: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3864, t3865, t3866) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk927::<F>(t3862, t555, t1361, t835, t1336);
        let (t3867, t3870, t3872) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk928::<F>(t1369, t3866, t1995, t241, t67, t3734, t820);
        let t3876 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk929::<F>(t1367, t3719, t820);
        let t3879 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk930::<F>(t1315, t1341, t1354, t1363, t1369, t3733, t3762, t3763, t3766, t3770, t3774, t3778, t3781, t3783, t3790, t3795, t3800, t3803, t3809, t3853, t3858, t3864, t3867, t3872, t3876, t559);
        let (t3880, t3882) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk931::<F>(t3879, t539, t1373, t225);
        let t3886 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk932::<F>(t1376, t566);
        let t3887 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk933::<F>(t3886, t68);
    (t3864, t3865, t3866, t3867, t3870, t3872, t3876, t3879, t3880, t3882, t3886, t3887)
}
