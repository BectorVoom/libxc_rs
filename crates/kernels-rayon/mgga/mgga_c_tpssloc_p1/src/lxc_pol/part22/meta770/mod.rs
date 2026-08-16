//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta770 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2621;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2622;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2623;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta770(t14730: f64, t17635: f64, t1193: f64, t22104: f64, t22038: f64, t3448: f64, t20234: f64, t44607: f64, t15376: f64, t18446: f64, t11569: f64, t15313: f64, t18410: f64, t18413: f64, t18420: f64, t18424: f64, t18428: f64, t18443: f64, t18466: f64, t18470: f64, t18475: f64, t3447: f64, t3451: f64, t4900: f64, t4905: f64, t4908: f64, t4909: f64, t64624: f64, t64627: f64, t64632: f64, t64811: f64, t71189: f64, t71197: f64, t71201: f64, t15338: f64, t18427: f64, t22032: f64, t11570: f64, t1409: f64, t15293: f64, t18416: f64, t18469: f64, t18542: f64, t3449: f64, t3450: f64, t4919: f64, t4928: f64, t52140: f64, t71168: f64, t71172: f64, t71181: f64, t71185: f64, t18457: f64, t4889: f64, t18321: f64, t4896: f64, t18451: f64, t1174: f64, t22081: f64, t44562: f64, t22046: f64, t3431: f64, t15281: f64, t22051: f64, t11539: f64, t22055: f64, t18454: f64, t1180: f64, t1184: f64, t1714: f64, t18523: f64, t18550: f64, t18555: f64, t460: f64, t4934: f64, t4937: f64, t6138: f64, t73113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73138, t73142, t73192) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2621(t14730, t17635, t1193, t22104, t22038, t3448, t20234, t44607, t15376, t18446, t11569, t15313, t18410, t18413, t18420, t18424, t18428, t18443, t18466, t18470, t18475, t3447, t3451, t4900, t4905, t4908, t4909, t64624, t64627, t64632, t64811, t71189, t71197, t71201);
        let (t73199, t73201, t73252) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2622(t15338, t18427, t3447, t22032, t3448, t11570, t20234, t1409, t15293, t18416, t18420, t18469, t18542, t3449, t3450, t4900, t4908, t4919, t4928, t52140, t71168, t71172, t71181, t71185, t73138);
        let (t73272, t73274, t73276, t73279, t73287, t73290) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2623(t18457, t4889, t18321, t4896, t18451, t1174, t22081, t44562, t22046, t3431, t15281, t22051);
        let t73316 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2624(t11539, t1174, t22055, t18454, t4889, t1180, t1184, t1714, t18321, t18523, t18550, t18555, t22032, t460, t4928, t4934, t4937, t6138, t73113, t73287, t73290);
    (t73138, t73142, t73192, t73199, t73201, t73252, t73272, t73274, t73276, t73279, t73316)
}
