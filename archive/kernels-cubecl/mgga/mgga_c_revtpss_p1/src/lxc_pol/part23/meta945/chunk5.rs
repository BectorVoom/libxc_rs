//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3109/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3109<F: Float>(t24362: F, t3479: F, t24407: F, t3523: F, t1168: F, t1187: F, t12470: F, t12486: F, t12553: F, t17097: F, t17154: F, t20625: F, t20665: F, t20668: F, t20672: F, t20675: F, t20679: F, t24330: F, t24376: F, t24408: F, t24411: F, t3477: F, t3496: F, t3521: F, t45157: F, t45159: F, t45177: F, t5142: F, t5163: F, t5180: F, t5185: F, t58247: F, t58262: F, t6538: F, t69359: F, t69371: F) -> F {
    let t81836 = t24362 * t3479;
    let t81873 = t24407 * t3523;
    let t81877 = F::cast_from(0.32163958997385070134e2_f64) * t3477 * t81836 * t1168 + F::cast_from(0.6207121550312808036e4_f64) * t12470 * t20625 * t5142 + F::cast_from(0.19964560303604640732e6_f64) * t45157 * t24330 * t45159 * t1168 - F::cast_from(0.35089341735807877242e1_f64) * t69371 * t5163 + F::cast_from(0.51947577317044391276e2_f64) * t69359 * t5185 - F::cast_from(0.35089341735807877242e1_f64) * t17154 * t20668 + F::cast_from(0.51947577317044391276e2_f64) * t17097 * t20672 - F::cast_from(0.31168546390226634765e3_f64) * t58262 * t20665 + F::cast_from(0.10389515463408878255e3_f64) * t17097 * t20675 + F::cast_from(0.30762056574649219972e4_f64) * t58247 * t20679 + F::cast_from(0.6233709278045326953e3_f64) * t12553 * t24376 * t1187 - F::cast_from(0.31168546390226634765e3_f64) * t12486 * t6538 * t5180 - F::cast_from(0.12304822629859687989e5_f64) * t45177 * t24411 * t1187 - F::cast_from(0.11696447245269292414e1_f64) * t3496 * t24408 * t1187 + F::cast_from(0.17315859105681463759e2_f64) * t3521 * t81873 * t1187;
    t81877
}
