//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3198/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3198(t17351: f64, t17353: f64, t20766: f64, t20767: f64, t20929: f64, t3611: f64, t44510: f64, t44521: f64, t44829: f64, t5406: f64, t57660: f64, t69832: f64, t71061: f64, t71373: f64, t71377: f64, t71400: f64, t71435: f64, t71447: f64, t71460: f64, t83760: f64) -> f64 {
    let t84020 = -t71373 / 72.0_f64 - t71377 / 48.0_f64 - 0.28582678745379824648e-3_f64 * t71400 - 0.63517063878621832551e-4_f64 * t44829 - 0.45732285992607719436e-2_f64 * t57660 * t20929 - 0.17149607247227894789e-2_f64 * t71447 * t20767 + 0.85748036236139473944e-3_f64 * t17351 * t17353 * t3611 * t83760 + 0.95275595817932748825e-3_f64 * t71435 - 0.85748036236139473944e-3_f64 * t44521 * t69832 * t5406 + 0.17149607247227894789e-2_f64 * t44510 * t71061 * t20766 + t71460 / 54.0_f64;
    t84020
}
