//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1939;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1940;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1941;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta529<F: Float>(t1065: F, t1409: F, t23330: F, t23329: F, t1945: F, t4552: F, t1603: F, t6768: F, t23384: F, t7557: F, t4693: F, t6705: F, t6704: F, t14555: F, t1635: F, t1956: F, t23327: F, t23369: F, t23392: F, t23579: F, t25798: F, t25802: F, t25807: F, t25811: F, t3169: F, t388: F, t4557: F, t6680: F, t6687: F, t6816: F, t7562: F, t7625: F, t25446: F, t25762: F, t25794: F, t3216: F, t7627: F, t1068: F, t1637: F, t1484: F, t1530: F, t16596: F, t1877: F, t1915: F, t193: F, t202: F, t23290: F, t23295: F, t2522: F, t25353: F, t25358: F, t25365: F, t25374: F, t4119: F, t4255: F, t4303: F, t4314: F, t6666: F, t6670: F, t7541: F, t776: F, t868: F, t870: F, t265: F, t394: F, t1070: F, t23738: F, t23742: F, t336: F, t4696: F, t4700: F, t6822: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25815, t25816, t25820, t25822, t25824, t25826) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1939::<F>(t1065, t1409, t23330, t23329, t1945, t4552, t1603, t6768, t23384, t7557, t4693, t6705);
        let (t25827, t25834) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1940::<F>(t25826, t6704, t14555, t1635, t1956, t23327, t23369, t23392, t23579, t25798, t25802, t25807, t25811, t25816, t25820, t25822, t25824, t3169, t388, t4557, t6680, t6687, t6816, t7562, t7625);
        let (t25836, t25840, t25845, t25882) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1941::<F>(t25446, t25762, t25794, t25834, t3216, t7627, t1068, t1637, t1484, t1530, t16596, t1877, t1915, t193, t202, t23290, t23295, t2522, t25353, t25358, t25365, t25374, t4119, t4255, t4303, t4314, t6666, t6670, t7541, t776, t868, t870);
        let t25883 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1942::<F>(t265, t394, t1068, t1070, t1637, t193, t23738, t23742, t25836, t25840, t25845, t25882, t336, t4696, t4700, t6822);
    (t25815, t25816, t25820, t25822, t25826, t25827, t25836, t25840, t25845, t25882, t25883)
}
