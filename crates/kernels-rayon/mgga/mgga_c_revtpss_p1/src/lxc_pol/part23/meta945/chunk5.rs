//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3109/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3109(t24362: f64, t3479: f64, t24407: f64, t3523: f64, t1168: f64, t1187: f64, t12470: f64, t12486: f64, t12553: f64, t17097: f64, t17154: f64, t20625: f64, t20665: f64, t20668: f64, t20672: f64, t20675: f64, t20679: f64, t24330: f64, t24376: f64, t24408: f64, t24411: f64, t3477: f64, t3496: f64, t3521: f64, t45157: f64, t45159: f64, t45177: f64, t5142: f64, t5163: f64, t5180: f64, t5185: f64, t58247: f64, t58262: f64, t6538: f64, t69359: f64, t69371: f64) -> f64 {
    let t81836 = t24362 * t3479;
    let t81873 = t24407 * t3523;
    let t81877 = 0.32163958997385070134e2_f64 * t3477 * t81836 * t1168 + 0.6207121550312808036e4_f64 * t12470 * t20625 * t5142 + 0.19964560303604640732e6_f64 * t45157 * t24330 * t45159 * t1168 - 0.35089341735807877242e1_f64 * t69371 * t5163 + 0.51947577317044391276e2_f64 * t69359 * t5185 - 0.35089341735807877242e1_f64 * t17154 * t20668 + 0.51947577317044391276e2_f64 * t17097 * t20672 - 0.31168546390226634765e3_f64 * t58262 * t20665 + 0.10389515463408878255e3_f64 * t17097 * t20675 + 0.30762056574649219972e4_f64 * t58247 * t20679 + 0.6233709278045326953e3_f64 * t12553 * t24376 * t1187 - 0.31168546390226634765e3_f64 * t12486 * t6538 * t5180 - 0.12304822629859687989e5_f64 * t45177 * t24411 * t1187 - 0.11696447245269292414e1_f64 * t3496 * t24408 * t1187 + 0.17315859105681463759e2_f64 * t3521 * t81873 * t1187;
    t81877
}
