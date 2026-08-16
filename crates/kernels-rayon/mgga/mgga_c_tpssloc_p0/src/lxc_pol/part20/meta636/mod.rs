//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2337;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2338;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2339;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2340;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2341;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta636(t10481: f64, t23508: f64, t10469: f64, t1603: f64, t11058: f64, t1625: f64, t11045: f64, t11064: f64, t1058: f64, t1060: f64, t10857: f64, t11028: f64, t11034: f64, t11040: f64, t11046: f64, t11048: f64, t11049: f64, t11061: f64, t11067: f64, t14608: f64, t14622: f64, t14654: f64, t3200: f64, t43480: f64, t43536: f64, t4669: f64, t4674: f64, t4677: f64, t4685: f64, t10236: f64, t14165: f64, t13831: f64, t13847: f64, t2986: f64, t10913: f64, t4337: f64, t10254: f64, t12648: f64, t43070: f64, t10190: f64, t13835: f64, t10186: f64, t10259: f64, t13832: f64, t13836: f64, t13839: f64, t13851: f64, t13934: f64, t2776: f64, t2780: f64, t2960: f64, t2988: f64, t42762: f64, t42773: f64, t42785: f64, t42788: f64, t42794: f64, t42846: f64, t43043: f64, t43069: f64, t4518: f64, t4531: f64, t6733: f64, t42841: f64, t12652: f64, t10241: f64, t13861: f64, t17748: f64, t42889: f64, t42893: f64, t42895: f64, t42903: f64, t42906: f64, t43065: f64, t47701: f64, t10255: f64, t13806: f64, t13871: f64, t42775: f64, t42964: f64, t42968: f64, t42974: f64, t4510: f64, t4514: f64, t47684: f64, t47726: f64, t47746: f64, t47767: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47819, t47840, t47844, t47867) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2337(t10481, t23508, t10469, t1603, t11058, t1625, t11045, t11064, t1058, t1060, t10857, t11028, t11034, t11040, t11046, t11048, t11049, t11061, t11067, t14608, t14622, t14654, t3200, t43480, t43536, t4669, t4674, t4677, t4685);
        let (t47887, t47907, t47915, t47919, t47927, t47938) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2338(t10236, t14165, t13831, t13847, t2986, t10913, t4337, t10254, t12648, t43070, t10190, t13835);
        let t47940 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2339(t10186, t10259, t13831, t13832, t13835, t13836, t13839, t13851, t13934, t2776, t2780, t2960, t2986, t2988, t42762, t42773, t42785, t42788, t42794, t42846, t43043, t43069, t4518, t4531, t47887, t47907, t47915, t47919, t47927, t47938, t6733);
        let (t47941, t47978) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2340(t14165, t42841, t10254, t12652, t10241, t10259, t13835, t13839, t13861, t17748, t2986, t2988, t42889, t42893, t42895, t42903, t42906, t43065, t4518, t47701);
        let t48017 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2341(t10186, t10255, t13806, t13851, t13871, t2986, t42775, t42964, t42968, t42974, t4510, t4514, t4518, t47684, t47726, t47746, t47767);
    (t47819, t47840, t47844, t47867, t47915, t47940, t47941, t47978, t48017)
}
