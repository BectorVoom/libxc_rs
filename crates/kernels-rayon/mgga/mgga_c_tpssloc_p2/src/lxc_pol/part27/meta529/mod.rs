//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1939;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1940;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1941;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta529(t1065: f64, t1409: f64, t23330: f64, t23329: f64, t1945: f64, t4552: f64, t1603: f64, t6768: f64, t23384: f64, t7557: f64, t4693: f64, t6705: f64, t6704: f64, t14555: f64, t1635: f64, t1956: f64, t23327: f64, t23369: f64, t23392: f64, t23579: f64, t25798: f64, t25802: f64, t25807: f64, t25811: f64, t3169: f64, t388: f64, t4557: f64, t6680: f64, t6687: f64, t6816: f64, t7562: f64, t7625: f64, t25446: f64, t25762: f64, t25794: f64, t3216: f64, t7627: f64, t1068: f64, t1637: f64, t1484: f64, t1530: f64, t16596: f64, t1877: f64, t1915: f64, t193: f64, t202: f64, t23290: f64, t23295: f64, t2522: f64, t25353: f64, t25358: f64, t25365: f64, t25374: f64, t4119: f64, t4255: f64, t4303: f64, t4314: f64, t6666: f64, t6670: f64, t7541: f64, t776: f64, t868: f64, t870: f64, t265: f64, t394: f64, t1070: f64, t23738: f64, t23742: f64, t336: f64, t4696: f64, t4700: f64, t6822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25815, t25816, t25820, t25822, t25824, t25826) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1939(t1065, t1409, t23330, t23329, t1945, t4552, t1603, t6768, t23384, t7557, t4693, t6705);
        let (t25827, t25834) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1940(t25826, t6704, t14555, t1635, t1956, t23327, t23369, t23392, t23579, t25798, t25802, t25807, t25811, t25816, t25820, t25822, t25824, t3169, t388, t4557, t6680, t6687, t6816, t7562, t7625);
        let (t25836, t25840, t25845, t25882) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1941(t25446, t25762, t25794, t25834, t3216, t7627, t1068, t1637, t1484, t1530, t16596, t1877, t1915, t193, t202, t23290, t23295, t2522, t25353, t25358, t25365, t25374, t4119, t4255, t4303, t4314, t6666, t6670, t7541, t776, t868, t870);
        let t25883 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1942(t265, t394, t1068, t1070, t1637, t193, t23738, t23742, t25836, t25840, t25845, t25882, t336, t4696, t4700, t6822);
    (t25815, t25816, t25820, t25822, t25826, t25827, t25836, t25840, t25845, t25882, t25883)
}
