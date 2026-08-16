//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1008/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1008(t8138: f64, t870: f64, t2273: f64, t3106: f64, t1185: f64, t2198: f64, t2240: f64, t1197: f64, t2252: f64, t2257: f64, t2274: f64, t2279: f64, t2282: f64, t2291: f64, t3083: f64, t3103: f64, t3107: f64, t3136: f64, t6303: f64, t6308: f64, t6313: f64, t8115: f64, t8120: f64, t8129: f64, t8132: f64, t8135: f64, t872: f64) -> (f64, f64, f64, f64, f64) {
    let t8139 = t8138 * t870;
    let t8142 = t3106 * t2273;
    let t8145 = t1185 * t2198;
    let t8147 = 6.0_f64 * t2240 * t8145;
    let t8148 = 0.11696447245269292414e1_f64 * t2291 * t3136 + 2.0_f64 * t8115 * t872 + 1.0_f64 * t3083 * t2274 + 0.32163958997385070134e2_f64 * t8120 * t2282 + 1.0_f64 * t6303 * t1197 + 2.0_f64 * t2252 * t3103 + 0.64327917994770140268e2_f64 * t6308 * t3107 - 4.0_f64 * t2257 * t8129 - 2.0_f64 * t2257 * t8132 - 0.19298375398431042081e3_f64 * t6313 * t8135 + 0.64327917994770140268e2_f64 * t2279 * t8139 + 0.32163958997385070134e2_f64 * t2279 * t8142 - t8147;
    (t8139, t8142, t8145, t8147, t8148)
}
