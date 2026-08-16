//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1328;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1329;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1330;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1331;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1332;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta377<F: Float>(t16804: F, t252: F, t1492: F, t4265: F, t225: F, t5632: F, t5561: F, t1519: F, t4142: F, t5631: F, t798: F, t5558: F, t852: F, t13042: F, t13053: F, t13065: F, t13463: F, t1528: F, t259: F, t2597: F, t4268: F, t4273: F, t5658: F, t866: F, t17079: F, t2752: F, t5660: F, t13105: F, t16685: F, t16688: F, t16691: F, t16692: F, t16695: F, t16696: F, t1877: F, t193: F, t202: F, t4303: F, t4307: F, t868: F, t870: F, t9789: F, t9793: F, t9797: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F, t10143: F, t5664: F, t12895: F, t13121: F, t1484: F, t16697: F, t16699: F, t16700: F, t16703: F, t16705: F, t16707: F, t16708: F, t16709: F, t16712: F, t16715: F, t16719: F, t2522: F, t262: F, t5527: F, t776: F, t9853: F, t9859: F, t9894: F, t9907: F, t9921: F, t16610: F, t16666: F, t1534: F, t2: F, t584: F, t5678: F, t690: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17083, t17087, t17090, t17092, t17095, t17098, t17100) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1328::<F>(t16804, t252, t1492, t4265, t225, t5632, t5561, t1519, t4142, t5631, t798, t5558, t852);
        let t17108 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1329::<F>(t13042, t13053, t13065, t13463, t1528, t17083, t17087, t17090, t17092, t17095, t17098, t17100, t259, t2597, t4268, t4273, t5658, t866);
        let (t17109, t17119) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1330::<F>(t17079, t17108, t2752, t5660, t13105, t16685, t16688, t16691, t16692, t16695, t16696, t1877, t193, t202, t4303, t4307, t868, t870, t9789, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890);
        let t17131 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1331::<F>(t10143, t5664, t12895, t13121, t1484, t16697, t16699, t16700, t16703, t16705, t16707, t16708, t16709, t16712, t16715, t16719, t1877, t193, t2522, t262, t5527, t776, t868, t9853, t9859, t9894, t9907, t9921);
        let (t17133, t17141, t17149) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1332::<F>(t16610, t16666, t17119, t17131, t1534, t2, t584, t5678, t690);
    (t17083, t17087, t17090, t17092, t17095, t17098, t17100, t17109, t17133, t17141, t17149)
}
