//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta688 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2265;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2266;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta688(t3263: f64, t5983: f64, t3331: f64, t6031: f64, t18785: f64, t3400: f64, t19262: f64, t3640: f64, t18287: f64, t225: f64, t15419: f64, t18215: f64, t3447: f64, t18469: f64, t44525: f64, t18206: f64, t52133: f64, t4899: f64, t6138: f64, t6144: f64, t15376: f64, t15420: f64, t18211: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64257, t64292, t64525, t64548, t64595, t64624) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2265(t3263, t5983, t3331, t6031, t18785, t3400, t19262, t3640, t18287, t225, t15419, t18215, t3447);
        let (t64627, t64632, t64644, t64648, t64667, t64686) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2266(t18469, t3447, t44525, t18206, t52133, t4899, t6138, t6144, t15376, t15420, t15419, t18211);
    (t64257, t64292, t64525, t64548, t64595, t64624, t64627, t64632, t64644, t64648, t64667, t64686)
}
