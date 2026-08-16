//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta770 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2621;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2622;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2623;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta770<F: Float>(t14730: F, t17635: F, t1193: F, t22104: F, t22038: F, t3448: F, t20234: F, t44607: F, t15376: F, t18446: F, t11569: F, t15313: F, t18410: F, t18413: F, t18420: F, t18424: F, t18428: F, t18443: F, t18466: F, t18470: F, t18475: F, t3447: F, t3451: F, t4900: F, t4905: F, t4908: F, t4909: F, t64624: F, t64627: F, t64632: F, t64811: F, t71189: F, t71197: F, t71201: F, t15338: F, t18427: F, t22032: F, t11570: F, t1409: F, t15293: F, t18416: F, t18469: F, t18542: F, t3449: F, t3450: F, t4919: F, t4928: F, t52140: F, t71168: F, t71172: F, t71181: F, t71185: F, t18457: F, t4889: F, t18321: F, t4896: F, t18451: F, t1174: F, t22081: F, t44562: F, t22046: F, t3431: F, t15281: F, t22051: F, t11539: F, t22055: F, t18454: F, t1180: F, t1184: F, t1714: F, t18523: F, t18550: F, t18555: F, t460: F, t4934: F, t4937: F, t6138: F, t73113: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t73138, t73142, t73192) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2621::<F>(t14730, t17635, t1193, t22104, t22038, t3448, t20234, t44607, t15376, t18446, t11569, t15313, t18410, t18413, t18420, t18424, t18428, t18443, t18466, t18470, t18475, t3447, t3451, t4900, t4905, t4908, t4909, t64624, t64627, t64632, t64811, t71189, t71197, t71201);
        let (t73199, t73201, t73252) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2622::<F>(t15338, t18427, t3447, t22032, t3448, t11570, t20234, t1409, t15293, t18416, t18420, t18469, t18542, t3449, t3450, t4900, t4908, t4919, t4928, t52140, t71168, t71172, t71181, t71185, t73138);
        let (t73272, t73274, t73276, t73279, t73287, t73290) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2623::<F>(t18457, t4889, t18321, t4896, t18451, t1174, t22081, t44562, t22046, t3431, t15281, t22051);
        let t73316 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2624::<F>(t11539, t1174, t22055, t18454, t4889, t1180, t1184, t1714, t18321, t18523, t18550, t18555, t22032, t460, t4928, t4934, t4937, t6138, t73113, t73287, t73290);
    (t73138, t73142, t73192, t73199, t73201, t73252, t73272, t73274, t73276, t73279, t73316)
}
