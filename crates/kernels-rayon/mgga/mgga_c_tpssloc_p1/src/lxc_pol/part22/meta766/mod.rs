//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta766 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2588;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2589;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2590;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta766(t22222: f64, t3411: f64, t14858: f64, t6106: f64, t1164: f64, t18275: f64, t21906: f64, t44154: f64, t21830: f64, t6098: f64, t22237: f64, t71876: f64, t71879: f64, t72098: f64, t72104: f64, t72106: f64, t71821: f64, t71850: f64, t71853: f64, t71855: f64, t71867: f64, t72081: f64, t72083: f64, t72086: f64, t72094: f64, t72096: f64, t72195: f64, t72196: f64, t72198: f64, t19026: f64, t4997: f64, t18975: f64, t5005: f64, t11719: f64, t22307: f64, t248: f64, t3570: f64, t11668: f64, t1213: f64, t1214: f64, t1737: f64, t19002: f64, t3577: f64, t4724: f64, t475: f64, t52879: f64, t6219: f64, t65479: f64, t65482: f64, t65485: f64, t65506: f64, t65957: f64, t72181: f64, t72183: f64, t15438: f64, t19095: f64, t19083: f64, t4993: f64, t18392: f64, t5024: f64, t1226: f64, t22115: f64, t11692: f64, t1174: f64, t1177: f64, t1232: f64, t15700: f64, t15740: f64, t1735: f64, t18221: f64, t18397: f64, t18401: f64, t19010: f64, t19106: f64, t3440: f64, t3578: f64, t4889: f64, t52766: f64, t53298: f64, t5392: f64, t65528: f64, t71172: f64, t71193: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t72201, t72203, t72207, t72209, t72211, t72213, t72214) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2588(t22222, t3411, t14858, t6106, t1164, t18275, t21906, t44154, t21830, t6098, t22237, t71876, t71879, t72098, t72104, t72106);
        let t72217 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2589(t71821, t71850, t71853, t71855, t71867, t72081, t72083, t72086, t72094, t72096, t72195, t72196, t72198, t72214);
        let t72233 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2590(t19026, t4997, t18975, t5005, t11719, t22307, t248, t3570, t11668, t1213, t1214, t1737, t19002, t3577, t4724, t475, t52879, t6219, t65479, t65482, t65485, t65506, t65957, t72181, t72183, t72217);
        let t72268 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2591(t15438, t19095, t19083, t4993, t18392, t5024, t1226, t22115, t11692, t1174, t1177, t1232, t15700, t15740, t1735, t18221, t18397, t18401, t19010, t19106, t3440, t3577, t3578, t4889, t52766, t53298, t5392, t65528, t71172, t71193);
    (t72201, t72203, t72207, t72209, t72211, t72213, t72217, t72233, t72268)
}
