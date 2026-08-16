//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta500 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1727;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1728;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1729;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1730;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1731;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta500(t193: f64, t2061: f64, t1877: f64, t2057: f64, t2219: f64, t1408: f64, t24191: f64, t24339: f64, t25: f64, t25015: f64, t25021: f64, t25024: f64, t25028: f64, t2522: f64, t25366: f64, t25375: f64, t25377: f64, t25381: f64, t25385: f64, t25392: f64, t26563: f64, t26740: f64, t26744: f64, t606: f64, t6542: f64, t6671: f64, t7110: f64, t7114: f64, t7475: f64, t7545: f64, t7845: f64, t1484: f64, t1530: f64, t16596: f64, t202: f64, t24344: f64, t25365: f64, t25374: f64, t26739: f64, t4119: f64, t4255: f64, t4303: f64, t4314: f64, t776: f64, t868: f64, t870: f64, t265: f64, t394: f64, t1409: f64, t2064: f64, t3966: f64, t40: f64, t607: f64, t7131: f64, t7865: f64, t1081: f64, t1649: f64, t25892: f64, t25898: f64, t25901: f64, t25905: f64, t25921: f64, t25928: f64, t25930: f64, t25934: f64, t25938: f64, t25945: f64, t28: f64, t6841: f64, t6848: f64, t7649: f64, t7656: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t504: f64, t2071: f64, t52: f64, t7150: f64, t7884: f64, t19577: f64, t24432: f64, rho1: f64, t5308: f64, t9016: f64, t15868: f64, t2095: f64, t5161: f64, t7217: f64, t113: f64, t19456: f64, t1983: f64, t2040: f64, t2096: f64, t22574: f64, t24987: f64, t24995: f64, t26161: f64, t26559: f64, t4028: f64, t6876: f64, t7050: f64, t7057: f64, t7171: f64, t7220: f64, t7685: f64, t7904: f64, t7943: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t26756 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1727(t193, t2061);
        let (t26774, t26775) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1728(t1877, t2057, t2219, t1408, t24191, t24339, t25, t25015, t25021, t25024, t25028, t2522, t25366, t25375, t25377, t25381, t25385, t25392, t26563, t26740, t26744, t26756, t606, t6542, t6671, t7110, t7114, t7475, t7545, t7845);
        let t26806 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1729(t1484, t1530, t16596, t1877, t193, t202, t2057, t24339, t24344, t2522, t25365, t25374, t26739, t26744, t4119, t4255, t4303, t4314, t7110, t7114, t776, t7845, t868, t870);
        let (t26807, t26814, t26861) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1730(t25, t265, t394, t26806, t1409, t2064, t26775, t3966, t40, t607, t7131, t7865, t1081, t1649, t1877, t2057, t24191, t24339, t2522, t25892, t25898, t25901, t25905, t25921, t25928, t25930, t25934, t25938, t25945, t26563, t26740, t26744, t26756, t26774, t28, t6841, t6848, t7110, t7114, t7649, t7656, t7845, dens_threshold, rho0, zeta_threshold);
        let (t26862, t26870, t26872) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1731(t28, t265, t504, t26806, t1409, t2071, t26861, t3966, t52, t607, t7150, t7884, t26814, t19577, t24432, dens_threshold, rho1, zeta_threshold);
        let (t26875, t26878, t26880, t26895) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1732(t5308, t9016, t15868, t2095, t5161, t7217, t113, t19456, t1983, t2040, t2096, t22574, t24987, t24995, t26161, t26559, t26870, t26872, t4028, t6876, t7050, t7057, t7171, t7220, t7685, t7904, t7943);
    (t26756, t26807, t26862, t26870, t26872, t26875, t26878, t26880, t26895)
}
