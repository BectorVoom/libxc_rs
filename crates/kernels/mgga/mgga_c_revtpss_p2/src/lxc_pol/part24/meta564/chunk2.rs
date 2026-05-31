//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1704/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1704<F: Float>(t89144: F, t89157: F, t1025: F, t11703: F, t1469: F, t15618: F, t16095: F, t1651: F, t1665: F, t18936: F, t19773: F, t225: F, t23913: F, t24024: F, t24034: F, t366: F, t371: F, t372: F, t373: F, t375: F, t4858: F, t53878: F, t54118: F, t6268: F, t6278: F, t6312: F, t65654: F, t67528: F, t79428: F, t79439: F, t79474: F, t79864: F, t88675: F) -> (F, F) {
    let t89158 = t89144 + t89157;
    let t89180 = F::cast_from(0.11433071498151929859e-2_f64) * t79428 + F::cast_from(0.11433071498151929859e-2_f64) * t79439 + F::cast_from(0.21437009059034868486e-3_f64) * t88675 * t225 * t366 * t375 - F::cast_from(0.22866142996303859718e-2_f64) * t79474 - F::cast_from(0.12862205435420921092e-2_f64) * t19773 * t6278 - F::cast_from(0.85748036236139473944e-3_f64) * t4858 * t24024 - F::cast_from(0.21437009059034868486e-3_f64) * t1025 * t371 * t372 * t373 * t89158 - F::cast_from(0.85748036236139473944e-3_f64) * t79864 * t1665 - F::cast_from(0.51448821741683684368e-2_f64) * t53878 * t24034 - F::cast_from(0.28582678745379824648e-2_f64) * t16095 * t11703 * t18936 * t1469 * t1651 + F::cast_from(0.17149607247227894789e-2_f64) * t15618 * t23913 + F::cast_from(0.17149607247227894789e-2_f64) * t67528 * t6268 + F::cast_from(5.0_f64) / F::cast_from(972.0_f64) * t54118 - F::cast_from(0.12862205435420921092e-2_f64) * t65654 * t6312;
    (t89158, t89180)
}
