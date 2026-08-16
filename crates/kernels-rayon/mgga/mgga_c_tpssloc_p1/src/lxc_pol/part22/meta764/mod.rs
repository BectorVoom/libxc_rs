//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta764 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2580;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2581;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2582;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2583;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta764(t71700: f64, t71704: f64, t71707: f64, t71711: f64, t71784: f64, t71786: f64, t71788: f64, t71790: f64, t71793: f64, t71795: f64, t71797: f64, t71800: f64, t71803: f64, t71806: f64, t71809: f64, t71811: f64, t71814: f64, t71817: f64, t71819: f64, t71821: f64, t71850: f64, t71853: f64, t18911: f64, t4869: f64, t18918: f64, t1164: f64, t4858: f64, t6105: f64, t1147: f64, t1156: f64, t71530: f64, t22229: f64, t3411: f64, t22233: f64, t1254: f64, t21994: f64, t43706: f64, t4700: f64, t71855: f64, t71867: f64, t71876: f64, t71879: f64, t1155: f64, t21906: f64, t43689: f64, t43692: f64, t18276: f64, t1238: f64, t1251: f64, t14972: f64, t1751: f64, t1761: f64, t18571: f64, t19209: f64, t19219: f64, t19234: f64, t19249: f64, t22004: f64, t22393: f64, t27784: f64, t3487: f64, t3598: f64, t4940: f64, t4945: f64, t498: f64, t5060: f64, t5089: f64, t53677: f64, t6238: f64, t6268: f64, t64595: f64, t65203: f64, t66845: f64, t66860: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t72077, t72078) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2580(t71700, t71704, t71707, t71711, t71784, t71786, t71788, t71790, t71793, t71795, t71797, t71800, t71803, t71806, t71809, t71811, t71814, t71817, t71819, t71821, t71850, t71853);
        let (t72081, t72083, t72086, t72094, t72096, t72098) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2581(t18911, t4869, t18918, t1164, t4858, t6105, t1147, t1156, t71530, t22229, t3411, t22233);
        let t72099 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2582(t1254, t21994, t43706, t4700, t71855, t71867, t71876, t71879, t72081, t72083, t72086, t72094, t72096, t72098);
        let (t72104, t72106, t72138) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2583(t1155, t1164, t21906, t43689, t43692, t18276, t4869, t1238, t1251, t14972, t1751, t1761, t18571, t19209, t19219, t19234, t19249, t22004, t22393, t27784, t3487, t3598, t4940, t4945, t498, t5060, t5089, t53677, t6238, t6268, t64595, t65203, t66845, t66860);
    (t72077, t72078, t72081, t72083, t72086, t72094, t72096, t72098, t72099, t72104, t72106, t72138)
}
