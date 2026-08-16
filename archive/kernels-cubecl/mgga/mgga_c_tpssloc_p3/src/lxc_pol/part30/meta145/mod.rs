//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta145 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk781;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk782;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk783;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk784;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk785;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta145<F: Float>(t1369: F, t3866: F, t1995: F, t241: F, t67: F, t1373: F, t225: F, t1376: F, t566: F, t68: F, t3787: F, t562: F, t1338: F, t1372: F, t193: F, t532: F, t1388: F, t1390: F, t531: F, t571: F, t112: F, t1395: F, t111: F, t576: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3867, t3870, t3882) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk781::<F>(t1369, t3866, t1995, t241, t67, t1373, t225);
        let t3886 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk782::<F>(t1376, t566);
        let t3887 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk783::<F>(t3886, t68);
        let (t3897, t3901, t3918) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk784::<F>(t3787, t562, t1338, t1372, t193, t532);
        let (t3919, t3924, t3938) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk785::<F>(t1388, t1390, t531, t571, t112, t1395);
        let t3941 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk786::<F>(t111, t576);
    (t3867, t3870, t3882, t3886, t3887, t3897, t3901, t3918, t3919, t3924, t3938, t3941)
}
