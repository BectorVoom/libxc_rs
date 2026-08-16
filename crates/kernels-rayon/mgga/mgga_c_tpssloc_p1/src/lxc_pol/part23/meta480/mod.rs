//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1437;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1438;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta480(t18915: f64, t6102: f64, t6274: f64, t3313: f64, t5989: f64, t6020: f64, t1703: f64, t71231: f64, t14838: f64, t21895: f64, t14850: f64, t21899: f64, t11190: f64, t6024: f64, t1670: f64, t21810: f64, t3264: f64, t71701: f64, t11275: f64, t18265: f64, t6267: f64, t15376: f64, t15395: f64, t18409: f64, t18416: f64, t18427: f64, t18469: f64, t22063: f64, t22066: f64, t3447: f64, t4919: f64, t52100: f64, t64644: f64, t73188: f64, t73199: f64, t73225: f64, t73272: f64, t73496: f64, t78035: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78344, t78348, t78355, t78357, t78359, t78361) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1437(t18915, t6102, t6274, t3313, t5989, t6020, t1703, t71231, t14838, t21895, t14850, t21899);
        let (t78364, t78367, t78370, t78373, t78379, t78423) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1438(t11190, t6020, t6024, t1670, t21810, t3264, t3313, t71701, t11275, t18265, t6267, t15376, t15395, t18409, t18416, t18427, t18469, t22063, t22066, t3447, t4919, t52100, t64644, t73188, t73199, t73225, t73272, t73496, t78035);
    (t78344, t78348, t78355, t78357, t78359, t78361, t78364, t78367, t78370, t78373, t78379, t78423)
}
