//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1704/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1704(t89144: f64, t89157: f64, t1025: f64, t11703: f64, t1469: f64, t15618: f64, t16095: f64, t1651: f64, t1665: f64, t18936: f64, t19773: f64, t225: f64, t23913: f64, t24024: f64, t24034: f64, t366: f64, t371: f64, t372: f64, t373: f64, t375: f64, t4858: f64, t53878: f64, t54118: f64, t6268: f64, t6278: f64, t6312: f64, t65654: f64, t67528: f64, t79428: f64, t79439: f64, t79474: f64, t79864: f64, t88675: f64) -> (f64, f64) {
    let t89158 = t89144 + t89157;
    let t89180 = 0.11433071498151929859e-2_f64 * t79428 + 0.11433071498151929859e-2_f64 * t79439 + 0.21437009059034868486e-3_f64 * t88675 * t225 * t366 * t375 - 0.22866142996303859718e-2_f64 * t79474 - 0.12862205435420921092e-2_f64 * t19773 * t6278 - 0.85748036236139473944e-3_f64 * t4858 * t24024 - 0.21437009059034868486e-3_f64 * t1025 * t371 * t372 * t373 * t89158 - 0.85748036236139473944e-3_f64 * t79864 * t1665 - 0.51448821741683684368e-2_f64 * t53878 * t24034 - 0.28582678745379824648e-2_f64 * t16095 * t11703 * t18936 * t1469 * t1651 + 0.17149607247227894789e-2_f64 * t15618 * t23913 + 0.17149607247227894789e-2_f64 * t67528 * t6268 + 5.0_f64 / 972.0_f64 * t54118 - 0.12862205435420921092e-2_f64 * t65654 * t6312;
    (t89158, t89180)
}
