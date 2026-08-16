//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta764 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2580;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2581;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2582;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2583;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta764<F: Float>(t71700: F, t71704: F, t71707: F, t71711: F, t71784: F, t71786: F, t71788: F, t71790: F, t71793: F, t71795: F, t71797: F, t71800: F, t71803: F, t71806: F, t71809: F, t71811: F, t71814: F, t71817: F, t71819: F, t71821: F, t71850: F, t71853: F, t18911: F, t4869: F, t18918: F, t1164: F, t4858: F, t6105: F, t1147: F, t1156: F, t71530: F, t22229: F, t3411: F, t22233: F, t1254: F, t21994: F, t43706: F, t4700: F, t71855: F, t71867: F, t71876: F, t71879: F, t1155: F, t21906: F, t43689: F, t43692: F, t18276: F, t1238: F, t1251: F, t14972: F, t1751: F, t1761: F, t18571: F, t19209: F, t19219: F, t19234: F, t19249: F, t22004: F, t22393: F, t27784: F, t3487: F, t3598: F, t4940: F, t4945: F, t498: F, t5060: F, t5089: F, t53677: F, t6238: F, t6268: F, t64595: F, t65203: F, t66845: F, t66860: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t72077, t72078) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2580::<F>(t71700, t71704, t71707, t71711, t71784, t71786, t71788, t71790, t71793, t71795, t71797, t71800, t71803, t71806, t71809, t71811, t71814, t71817, t71819, t71821, t71850, t71853);
        let (t72081, t72083, t72086, t72094, t72096, t72098) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2581::<F>(t18911, t4869, t18918, t1164, t4858, t6105, t1147, t1156, t71530, t22229, t3411, t22233);
        let t72099 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2582::<F>(t1254, t21994, t43706, t4700, t71855, t71867, t71876, t71879, t72081, t72083, t72086, t72094, t72096, t72098);
        let (t72104, t72106, t72138) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2583::<F>(t1155, t1164, t21906, t43689, t43692, t18276, t4869, t1238, t1251, t14972, t1751, t1761, t18571, t19209, t19219, t19234, t19249, t22004, t22393, t27784, t3487, t3598, t4940, t4945, t498, t5060, t5089, t53677, t6238, t6268, t64595, t65203, t66845, t66860);
    (t72077, t72078, t72081, t72083, t72086, t72094, t72096, t72098, t72099, t72104, t72106, t72138)
}
