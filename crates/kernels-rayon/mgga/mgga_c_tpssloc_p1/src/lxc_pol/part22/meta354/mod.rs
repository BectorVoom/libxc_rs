//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1568;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1569;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1570;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1571;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1572;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta354(t1527: f64, t4300: f64, t2718: f64, t17050: f64, t17052: f64, t17057: f64, t17060: f64, t17064: f64, t259: f64, t2597: f64, t2713: f64, t4147: f64, t4268: f64, t4273: f64, t4301: f64, t5637: f64, t5658: f64, t855: f64, t866: f64, t16804: f64, t252: f64, t1492: f64, t4265: f64, t225: f64, t5632: f64, t5561: f64, t1519: f64, t4142: f64, t5631: f64, t798: f64, t5558: f64, t852: f64, t13042: f64, t13053: f64, t13065: f64, t13463: f64, t1528: f64, t2752: f64, t5660: f64, t13105: f64, t16685: f64, t16688: f64, t16691: f64, t16692: f64, t16695: f64, t16696: f64, t1877: f64, t193: f64, t202: f64, t4303: f64, t4307: f64, t868: f64, t870: f64, t9789: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17070, t17079) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1568(t1527, t4300, t2718, t17050, t17052, t17057, t17060, t17064, t259, t2597, t2713, t4147, t4268, t4273, t4301, t5637, t5658, t855, t866);
        let (t17083, t17087, t17090, t17092, t17095, t17098, t17100) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1569(t16804, t252, t1492, t4265, t225, t5632, t5561, t1519, t4142, t5631, t798, t5558, t852);
        let t17108 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1570(t13042, t13053, t13065, t13463, t1528, t17083, t17087, t17090, t17092, t17095, t17098, t17100, t259, t2597, t4268, t4273, t5658, t866);
        let (t17109, t17116) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1571(t17079, t17108, t2752, t5660);
        let t17119 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1572(t13105, t16685, t16688, t16691, t16692, t16695, t16696, t17109, t17116, t1877, t193, t202, t4303, t4307, t868, t870, t9789, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890);
    (t17070, t17083, t17087, t17090, t17092, t17095, t17098, t17100, t17109, t17116, t17119)
}
