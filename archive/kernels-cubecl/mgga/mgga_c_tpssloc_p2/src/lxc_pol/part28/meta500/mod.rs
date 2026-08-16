//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta500 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1727;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1728;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1729;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1730;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1731;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta500<F: Float>(t193: F, t2061: F, t1877: F, t2057: F, t2219: F, t1408: F, t24191: F, t24339: F, t25: F, t25015: F, t25021: F, t25024: F, t25028: F, t2522: F, t25366: F, t25375: F, t25377: F, t25381: F, t25385: F, t25392: F, t26563: F, t26740: F, t26744: F, t606: F, t6542: F, t6671: F, t7110: F, t7114: F, t7475: F, t7545: F, t7845: F, t1484: F, t1530: F, t16596: F, t202: F, t24344: F, t25365: F, t25374: F, t26739: F, t4119: F, t4255: F, t4303: F, t4314: F, t776: F, t868: F, t870: F, t265: F, t394: F, t1409: F, t2064: F, t3966: F, t40: F, t607: F, t7131: F, t7865: F, t1081: F, t1649: F, t25892: F, t25898: F, t25901: F, t25905: F, t25921: F, t25928: F, t25930: F, t25934: F, t25938: F, t25945: F, t28: F, t6841: F, t6848: F, t7649: F, t7656: F, dens_threshold: F, rho0: F, zeta_threshold: F, t504: F, t2071: F, t52: F, t7150: F, t7884: F, t19577: F, t24432: F, rho1: F, t5308: F, t9016: F, t15868: F, t2095: F, t5161: F, t7217: F, t113: F, t19456: F, t1983: F, t2040: F, t2096: F, t22574: F, t24987: F, t24995: F, t26161: F, t26559: F, t4028: F, t6876: F, t7050: F, t7057: F, t7171: F, t7220: F, t7685: F, t7904: F, t7943: F) -> (F, F, F, F, F, F, F, F, F) {
        let t26756 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1727::<F>(t193, t2061);
        let (t26774, t26775) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1728::<F>(t1877, t2057, t2219, t1408, t24191, t24339, t25, t25015, t25021, t25024, t25028, t2522, t25366, t25375, t25377, t25381, t25385, t25392, t26563, t26740, t26744, t26756, t606, t6542, t6671, t7110, t7114, t7475, t7545, t7845);
        let t26806 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1729::<F>(t1484, t1530, t16596, t1877, t193, t202, t2057, t24339, t24344, t2522, t25365, t25374, t26739, t26744, t4119, t4255, t4303, t4314, t7110, t7114, t776, t7845, t868, t870);
        let (t26807, t26814, t26861) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1730::<F>(t25, t265, t394, t26806, t1409, t2064, t26775, t3966, t40, t607, t7131, t7865, t1081, t1649, t1877, t2057, t24191, t24339, t2522, t25892, t25898, t25901, t25905, t25921, t25928, t25930, t25934, t25938, t25945, t26563, t26740, t26744, t26756, t26774, t28, t6841, t6848, t7110, t7114, t7649, t7656, t7845, dens_threshold, rho0, zeta_threshold);
        let (t26862, t26870, t26872) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1731::<F>(t28, t265, t504, t26806, t1409, t2071, t26861, t3966, t52, t607, t7150, t7884, t26814, t19577, t24432, dens_threshold, rho1, zeta_threshold);
        let (t26875, t26878, t26880, t26895) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1732::<F>(t5308, t9016, t15868, t2095, t5161, t7217, t113, t19456, t1983, t2040, t2096, t22574, t24987, t24995, t26161, t26559, t26870, t26872, t4028, t6876, t7050, t7057, t7171, t7220, t7685, t7904, t7943);
    (t26756, t26807, t26862, t26870, t26872, t26875, t26878, t26880, t26895)
}
