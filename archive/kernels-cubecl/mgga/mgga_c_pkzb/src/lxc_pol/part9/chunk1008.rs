//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1008/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1008<F: Float>(t8138: F, t870: F, t2273: F, t3106: F, t1185: F, t2198: F, t2240: F, t1197: F, t2252: F, t2257: F, t2274: F, t2279: F, t2282: F, t2291: F, t3083: F, t3103: F, t3107: F, t3136: F, t6303: F, t6308: F, t6313: F, t8115: F, t8120: F, t8129: F, t8132: F, t8135: F, t872: F) -> (F, F, F, F, F) {
    let t8139 = t8138 * t870;
    let t8142 = t3106 * t2273;
    let t8145 = t1185 * t2198;
    let t8147 = F::cast_from(6.0_f64) * t2240 * t8145;
    let t8148 = F::cast_from(0.11696447245269292414e1_f64) * t2291 * t3136 + F::cast_from(2.0_f64) * t8115 * t872 + F::cast_from(1.0_f64) * t3083 * t2274 + F::cast_from(0.32163958997385070134e2_f64) * t8120 * t2282 + F::cast_from(1.0_f64) * t6303 * t1197 + F::cast_from(2.0_f64) * t2252 * t3103 + F::cast_from(0.64327917994770140268e2_f64) * t6308 * t3107 - F::cast_from(4.0_f64) * t2257 * t8129 - F::cast_from(2.0_f64) * t2257 * t8132 - F::cast_from(0.19298375398431042081e3_f64) * t6313 * t8135 + F::cast_from(0.64327917994770140268e2_f64) * t2279 * t8139 + F::cast_from(0.32163958997385070134e2_f64) * t2279 * t8142 - t8147;
    (t8139, t8142, t8145, t8147, t8148)
}
