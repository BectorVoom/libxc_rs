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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2153;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2154;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2155;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2156;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2157;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2158;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2159;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2160;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta684(t26308: f64, t5234: f64, t5252: f64, t6396: f64, t80820: f64, t19962: f64, t22833: f64, t19894: f64, t19886: f64, t5293: f64, t91100: f64, t19991: f64, t19882: f64, t91114: f64, t91121: f64, t97202: f64, t97204: f64, t97206: f64, t97208: f64, t97210: f64, t97212: f64, t97214: f64, t16311: f64, t3788: f64, t5286: f64, t6936: f64, t28101: f64, t80958: f64, t1827: f64, t91285: f64, t22756: f64, t6417: f64, t19868: f64, t6945: f64, t19815: f64, t6944: f64, t1354: f64, t91278: f64, t26233: f64, t5289: f64, t22765: f64, t6422: f64, t19921: f64, t6952: f64, t19926: f64, t22783: f64, t6431: f64, t1831: f64, t91160: f64, t6951: f64, t1369: f64, t91136: f64, t91138: f64, t91141: f64, t1339: f64, t1824: f64, t22827: f64, t5187: f64, t550: f64, t74677: f64, t1307: f64, t6388: f64, t6427: f64, t26288: f64, t57172: f64, t74366: f64, t6415: f64, t6420: f64, t1825: f64, t57091: f64, t91144: f64, t91155: f64, t91159: f64, t91162: f64, t91171: f64, t91180: f64, t93650: f64, t93656: f64, t19890: f64, t26309: f64, t236: f64, t6387: f64, t22705: f64, t22852: f64, t19805: f64, t2002: f64, t559: f64, t19986: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97217, t97219, t97221, t97223, t97225, t97227, t97229) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2153(t26308, t5234, t5252, t6396, t80820, t19962, t22833, t19894, t19886, t5293, t91100, t19991);
        let t97233 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2154(t19882, t22833, t91114, t91121, t97202, t97204, t97206, t97208, t97210, t97212, t97214, t97217, t97219, t97221, t97223, t97225, t97227, t97229);
        let (t97236, t97238, t97240, t97242, t97244) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2155(t16311, t3788, t5286, t6936, t28101, t80958, t1827, t91285, t22756, t6417, t19868, t6945);
        let (t97247, t97249, t97251, t97253, t97255, t97257) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2156(t19815, t6944, t1354, t1827, t91278, t26233, t5289, t22765, t6422, t19921, t6952, t19926);
        let t97268 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2157(t22756, t6422, t22783, t6431, t1831, t91160, t19815, t6951, t1369, t91136, t91138, t91141, t97236, t97238, t97240, t97242, t97244, t97247, t97249, t97251, t97253, t97255, t97257);
        let (t97273, t97277, t97281, t97283, t97287) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2158(t1339, t1824, t22827, t5187, t550, t74677, t1307, t3788, t6388, t22783, t6427, t26288, t57172);
        let t97309 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2159(t1339, t22827, t550, t74366, t1307, t6415, t6420, t1825, t5286, t6936, t57091, t91144, t91155, t91159, t91162, t91171, t91180, t93650, t93656, t97273, t97277, t97281, t97283, t97287);
        let (t97310, t97312, t97315, t97318, t97320) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2160(t19890, t26309, t236, t6387, t22705, t22852, t550, t19805, t2002, t559, t19986, t22833);
    (t97233, t97268, t97309, t97310, t97312, t97315, t97318, t97320)
}
