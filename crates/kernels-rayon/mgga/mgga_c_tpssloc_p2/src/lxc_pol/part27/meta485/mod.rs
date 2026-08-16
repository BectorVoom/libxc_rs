//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1862;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1863;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1864;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1865;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta485(t23664: f64, t23720: f64, t1055: f64, t1065: f64, t6815: f64, t3174: f64, t2780: f64, t6690: f64, t6689: f64, t10170: f64, t1052: f64, t11010: f64, t1956: f64, t23579: f64, t23582: f64, t23589: f64, t23595: f64, t3026: f64, t6680: f64, t6687: f64, t6700: f64, t6816: f64, t1923: f64, t23310: f64, t23314: f64, t23317: f64, t23323: f64, t23327: f64, t23333: f64, t23337: f64, t23341: f64, t23346: f64, t23381: f64, t23574: f64, t3169: f64, t6707: f64, t6776: f64, t3216: f64, t6818: f64, t11094: f64, t1958: f64, t13487: f64, t1877: f64, t1915: f64, t193: f64, t202: f64, t23285: f64, t23290: f64, t23295: f64, t2379: f64, t2522: f64, t2553: f64, t2745: f64, t2749: f64, t4314: f64, t6666: f64, t6670: f64, t776: f64, t868: f64, t870: f64, t265: f64, t394: f64, t1068: f64, t1070: f64, t3209: f64, t3213: f64, t336: f64, t4700: f64, t6822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23721, t23722, t23725, t23728, t23729, t23732) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1862(t23664, t23720, t1055, t1065, t6815, t3174, t2780, t6690, t6689, t10170, t1052, t11010, t1956, t23579, t23582, t23589, t23595, t3026, t6680, t6687, t6700, t6816);
        let t23734 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1863(t1052, t1923, t23310, t23314, t23317, t23323, t23327, t23333, t23337, t23341, t23346, t23381, t23574, t23732, t3026, t3169, t6687, t6707, t6776);
        let (t23738, t23742, t23772) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1864(t3216, t6818, t11094, t1958, t13487, t1877, t1915, t193, t202, t23285, t23290, t23295, t2379, t2522, t2553, t2745, t2749, t4314, t6666, t6670, t776, t868, t870);
        let t23773 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1865(t265, t394, t1068, t1070, t193, t23734, t23738, t23742, t23772, t3209, t3213, t336, t4700, t6822);
    (t23721, t23722, t23725, t23728, t23729, t23734, t23738, t23742, t23772, t23773)
}
