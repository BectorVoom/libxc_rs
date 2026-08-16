//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2005/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2005(t113: f64, t12725: f64, t12823: f64, t1393: f64, t1459: f64, t1774: f64, t1849: f64, t1983: f64, t2094: f64, t22574: f64, t23941: f64, t24026: f64, t24166: f64, t24167: f64, t24432: f64, t24987: f64, t26870: f64, t26880: f64, t26974: f64, t27144: f64, t27163: f64, t27215: f64, t3734: f64, t4026: f64, t4034: f64, t510: f64, t5161: f64, t56198: f64, t650: f64, t6876: f64, t6999: f64, t7061: f64, t7156: f64, t7218: f64, t7685: f64, t7687: f64, t7796: f64, t83886: f64, t84097: f64, t92073: f64, t93113: f64, t93261: f64) -> f64 {
    let t93275 = -t1983 * t24166 * t5161 - 2.0_f64 * t23941 * t1774 - t92073 * t510 + 2.0_f64 * t27215 * t1393 + t24026 * t1849 - 2.0_f64 * t650 * t26870 - 4.0_f64 * t12725 * t7061 - 2.0_f64 * t12823 * t7796 - 4.0_f64 * t4034 * t27163 + 2.0_f64 * t24987 * t7218 + t7685 * t24167 + 6.0_f64 * t1983 * t3734 * t2094 * t7687 - 6.0_f64 * t22574 * t24432 * t56198 - t113 * (t93113 + t93261) - 6.0_f64 * t83886 * t26974 - 2.0_f64 * t1983 * t27144 * t6999 - 2.0_f64 * t84097 * t1459 - 2.0_f64 * t6876 * t26880 - 2.0_f64 * t4026 * t7156;
    t93275
}
