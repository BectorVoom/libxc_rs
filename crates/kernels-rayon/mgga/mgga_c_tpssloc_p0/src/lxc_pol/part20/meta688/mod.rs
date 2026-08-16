//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta688 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2605;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2606;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2607;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2608;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2609;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta688(t3575: f64, t373: f64, t470: f64, t493: f64, t1214: f64, t820: f64, t3624: f64, t52627: f64, t11745: f64, t15503: f64, t15737: f64, t3493: f64, t475: f64, t607: f64, t1227: f64, t13969: f64, t15649: f64, t43763: f64, t44827: f64, t11539: f64, t1174: f64, t14740: f64, t11694: f64, t11739: f64, t1215: f64, t1653: f64, t3577: f64, t3578: f64, t44936: f64, t45119: f64, t45128: f64, t45197: f64, t4582: f64, t4728: f64, t48497: f64, t52183: f64, t52704: f64, t14731: f64, t135: f64, t15666: f64, t11665: f64, t15572: f64, t3515: f64, t4983: f64, t49850: f64, t11818: f64, t1213: f64, t248: f64, t5012: f64, t11801: f64, t5024: f64, t11820: f64, t5019: f64, t11729: f64, t15527: f64, t15541: f64, t15545: f64, t15656: f64, t3490: f64, t3536: f64, t44836: f64, t45037: f64, t45997: f64, t46006: f64, t4977: f64, t4987: f64, t11791: f64, t5002: f64, t11153: f64, t4899: f64, t3540: f64, t4961: f64, t11709: f64, t15640: f64, t11738: f64, t15535: f64, t15553: f64, t3447: f64, t3516: f64, t44965: f64, t44968: f64, t44972: f64, t44976: f64, t44982: f64, t45971: f64, t15611: f64, t15454: f64, t4973: f64, t11662: f64, t15478: f64, t44985: f64, t44988: f64, t44991: f64, t44994: f64, t44996: f64, t4950: f64, t51002: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52893, t52897, t52903, t52906, t52908, t52911) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2605(t3575, t373, t470, t493, t1214, t820, t3624, t52627, t11745, t15503, t15737, t3493, t475, t607);
        let t52928 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2606(t1227, t13969, t15649, t43763, t44827, t11539, t1174, t14740, t11694, t11739, t1215, t1653, t3493, t3577, t3578, t44936, t45119, t45128, t45197, t4582, t4728, t48497, t52183, t52704, t52893, t52897, t52903, t52906, t52908, t52911);
        let (t52932, t52935, t52942, t52953, t52973) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2607(t11539, t1174, t14731, t135, t15666, t11665, t15572, t3515, t4983, t49850, t11818, t1213, t248, t5012);
        let t52989 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2608(t52973, t11801, t5024, t11820, t5019, t11729, t11739, t1227, t15527, t15541, t15545, t15656, t3490, t3536, t44836, t45037, t4582, t45997, t46006, t4977, t4987);
        let t53013 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2609(t11791, t5024, t11820, t5002, t11153, t4899, t3540, t4961, t11709, t15640, t11738, t15535, t15553, t3447, t3516, t44965, t44968, t44972, t44976, t44982, t4582, t45971);
        let t53037 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2610(t1227, t13969, t15611, t15454, t4973, t49850, t11662, t11665, t15478, t15737, t44985, t44988, t44991, t44994, t44996, t4582, t48497, t4950, t51002);
    (t52893, t52897, t52911, t52928, t52932, t52935, t52942, t52953, t52989, t53013, t53037)
}
