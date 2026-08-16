//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1328;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1329;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1330;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1331;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1332;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta377(t16804: f64, t252: f64, t1492: f64, t4265: f64, t225: f64, t5632: f64, t5561: f64, t1519: f64, t4142: f64, t5631: f64, t798: f64, t5558: f64, t852: f64, t13042: f64, t13053: f64, t13065: f64, t13463: f64, t1528: f64, t259: f64, t2597: f64, t4268: f64, t4273: f64, t5658: f64, t866: f64, t17079: f64, t2752: f64, t5660: f64, t13105: f64, t16685: f64, t16688: f64, t16691: f64, t16692: f64, t16695: f64, t16696: f64, t1877: f64, t193: f64, t202: f64, t4303: f64, t4307: f64, t868: f64, t870: f64, t9789: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64, t10143: f64, t5664: f64, t12895: f64, t13121: f64, t1484: f64, t16697: f64, t16699: f64, t16700: f64, t16703: f64, t16705: f64, t16707: f64, t16708: f64, t16709: f64, t16712: f64, t16715: f64, t16719: f64, t2522: f64, t262: f64, t5527: f64, t776: f64, t9853: f64, t9859: f64, t9894: f64, t9907: f64, t9921: f64, t16610: f64, t16666: f64, t1534: f64, t2: f64, t584: f64, t5678: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17083, t17087, t17090, t17092, t17095, t17098, t17100) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1328(t16804, t252, t1492, t4265, t225, t5632, t5561, t1519, t4142, t5631, t798, t5558, t852);
        let t17108 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1329(t13042, t13053, t13065, t13463, t1528, t17083, t17087, t17090, t17092, t17095, t17098, t17100, t259, t2597, t4268, t4273, t5658, t866);
        let (t17109, t17119) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1330(t17079, t17108, t2752, t5660, t13105, t16685, t16688, t16691, t16692, t16695, t16696, t1877, t193, t202, t4303, t4307, t868, t870, t9789, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890);
        let t17131 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1331(t10143, t5664, t12895, t13121, t1484, t16697, t16699, t16700, t16703, t16705, t16707, t16708, t16709, t16712, t16715, t16719, t1877, t193, t2522, t262, t5527, t776, t868, t9853, t9859, t9894, t9907, t9921);
        let (t17133, t17141, t17149) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1332(t16610, t16666, t17119, t17131, t1534, t2, t584, t5678, t690);
    (t17083, t17087, t17090, t17092, t17095, t17098, t17100, t17109, t17133, t17141, t17149)
}
