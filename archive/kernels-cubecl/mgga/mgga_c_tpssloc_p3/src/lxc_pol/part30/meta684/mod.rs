//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta684 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2153;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2154;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2155;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2156;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2157;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2158;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2159;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2160;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta684<F: Float>(t26308: F, t5234: F, t5252: F, t6396: F, t80820: F, t19962: F, t22833: F, t19894: F, t19886: F, t5293: F, t91100: F, t19991: F, t19882: F, t91114: F, t91121: F, t97202: F, t97204: F, t97206: F, t97208: F, t97210: F, t97212: F, t97214: F, t16311: F, t3788: F, t5286: F, t6936: F, t28101: F, t80958: F, t1827: F, t91285: F, t22756: F, t6417: F, t19868: F, t6945: F, t19815: F, t6944: F, t1354: F, t91278: F, t26233: F, t5289: F, t22765: F, t6422: F, t19921: F, t6952: F, t19926: F, t22783: F, t6431: F, t1831: F, t91160: F, t6951: F, t1369: F, t91136: F, t91138: F, t91141: F, t1339: F, t1824: F, t22827: F, t5187: F, t550: F, t74677: F, t1307: F, t6388: F, t6427: F, t26288: F, t57172: F, t74366: F, t6415: F, t6420: F, t1825: F, t57091: F, t91144: F, t91155: F, t91159: F, t91162: F, t91171: F, t91180: F, t93650: F, t93656: F, t19890: F, t26309: F, t236: F, t6387: F, t22705: F, t22852: F, t19805: F, t2002: F, t559: F, t19986: F) -> (F, F, F, F, F, F, F, F) {
        let (t97217, t97219, t97221, t97223, t97225, t97227, t97229) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2153::<F>(t26308, t5234, t5252, t6396, t80820, t19962, t22833, t19894, t19886, t5293, t91100, t19991);
        let t97233 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2154::<F>(t19882, t22833, t91114, t91121, t97202, t97204, t97206, t97208, t97210, t97212, t97214, t97217, t97219, t97221, t97223, t97225, t97227, t97229);
        let (t97236, t97238, t97240, t97242, t97244) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2155::<F>(t16311, t3788, t5286, t6936, t28101, t80958, t1827, t91285, t22756, t6417, t19868, t6945);
        let (t97247, t97249, t97251, t97253, t97255, t97257) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2156::<F>(t19815, t6944, t1354, t1827, t91278, t26233, t5289, t22765, t6422, t19921, t6952, t19926);
        let t97268 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2157::<F>(t22756, t6422, t22783, t6431, t1831, t91160, t19815, t6951, t1369, t91136, t91138, t91141, t97236, t97238, t97240, t97242, t97244, t97247, t97249, t97251, t97253, t97255, t97257);
        let (t97273, t97277, t97281, t97283, t97287) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2158::<F>(t1339, t1824, t22827, t5187, t550, t74677, t1307, t3788, t6388, t22783, t6427, t26288, t57172);
        let t97309 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2159::<F>(t1339, t22827, t550, t74366, t1307, t6415, t6420, t1825, t5286, t6936, t57091, t91144, t91155, t91159, t91162, t91171, t91180, t93650, t93656, t97273, t97277, t97281, t97283, t97287);
        let (t97310, t97312, t97315, t97318, t97320) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2160::<F>(t19890, t26309, t236, t6387, t22705, t22852, t550, t19805, t2002, t559, t19986, t22833);
    (t97233, t97268, t97309, t97310, t97312, t97315, t97318, t97320)
}
