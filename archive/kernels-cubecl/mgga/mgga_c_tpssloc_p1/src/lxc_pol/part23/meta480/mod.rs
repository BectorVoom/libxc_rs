//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1437;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1438;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta480<F: Float>(t18915: F, t6102: F, t6274: F, t3313: F, t5989: F, t6020: F, t1703: F, t71231: F, t14838: F, t21895: F, t14850: F, t21899: F, t11190: F, t6024: F, t1670: F, t21810: F, t3264: F, t71701: F, t11275: F, t18265: F, t6267: F, t15376: F, t15395: F, t18409: F, t18416: F, t18427: F, t18469: F, t22063: F, t22066: F, t3447: F, t4919: F, t52100: F, t64644: F, t73188: F, t73199: F, t73225: F, t73272: F, t73496: F, t78035: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t78344, t78348, t78355, t78357, t78359, t78361) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1437::<F>(t18915, t6102, t6274, t3313, t5989, t6020, t1703, t71231, t14838, t21895, t14850, t21899);
        let (t78364, t78367, t78370, t78373, t78379, t78423) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1438::<F>(t11190, t6020, t6024, t1670, t21810, t3264, t3313, t71701, t11275, t18265, t6267, t15376, t15395, t18409, t18416, t18427, t18469, t22063, t22066, t3447, t4919, t52100, t64644, t73188, t73199, t73225, t73272, t73496, t78035);
    (t78344, t78348, t78355, t78357, t78359, t78361, t78364, t78367, t78370, t78373, t78379, t78423)
}
