//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta766 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2588;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2589;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2590;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta766<F: Float>(t22222: F, t3411: F, t14858: F, t6106: F, t1164: F, t18275: F, t21906: F, t44154: F, t21830: F, t6098: F, t22237: F, t71876: F, t71879: F, t72098: F, t72104: F, t72106: F, t71821: F, t71850: F, t71853: F, t71855: F, t71867: F, t72081: F, t72083: F, t72086: F, t72094: F, t72096: F, t72195: F, t72196: F, t72198: F, t19026: F, t4997: F, t18975: F, t5005: F, t11719: F, t22307: F, t248: F, t3570: F, t11668: F, t1213: F, t1214: F, t1737: F, t19002: F, t3577: F, t4724: F, t475: F, t52879: F, t6219: F, t65479: F, t65482: F, t65485: F, t65506: F, t65957: F, t72181: F, t72183: F, t15438: F, t19095: F, t19083: F, t4993: F, t18392: F, t5024: F, t1226: F, t22115: F, t11692: F, t1174: F, t1177: F, t1232: F, t15700: F, t15740: F, t1735: F, t18221: F, t18397: F, t18401: F, t19010: F, t19106: F, t3440: F, t3578: F, t4889: F, t52766: F, t53298: F, t5392: F, t65528: F, t71172: F, t71193: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t72201, t72203, t72207, t72209, t72211, t72213, t72214) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2588::<F>(t22222, t3411, t14858, t6106, t1164, t18275, t21906, t44154, t21830, t6098, t22237, t71876, t71879, t72098, t72104, t72106);
        let t72217 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2589::<F>(t71821, t71850, t71853, t71855, t71867, t72081, t72083, t72086, t72094, t72096, t72195, t72196, t72198, t72214);
        let t72233 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2590::<F>(t19026, t4997, t18975, t5005, t11719, t22307, t248, t3570, t11668, t1213, t1214, t1737, t19002, t3577, t4724, t475, t52879, t6219, t65479, t65482, t65485, t65506, t65957, t72181, t72183, t72217);
        let t72268 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2591::<F>(t15438, t19095, t19083, t4993, t18392, t5024, t1226, t22115, t11692, t1174, t1177, t1232, t15700, t15740, t1735, t18221, t18397, t18401, t19010, t19106, t3440, t3577, t3578, t4889, t52766, t53298, t5392, t65528, t71172, t71193);
    (t72201, t72203, t72207, t72209, t72211, t72213, t72217, t72233, t72268)
}
