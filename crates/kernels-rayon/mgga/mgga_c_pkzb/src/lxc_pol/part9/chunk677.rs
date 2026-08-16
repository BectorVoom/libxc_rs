//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 677/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk677(t3139: f64, t889: f64, t1197: f64, t1209: f64, t2252: f64, t2257: f64, t2279: f64, t2291: f64, t2296: f64, t2318: f64, t3032: f64, t3035: f64, t3037: f64, t3040: f64, t3072: f64, t3076: f64, t3080: f64, t3083: f64, t3088: f64, t3103: f64, t3107: f64, t3114: f64, t3116: f64, t3121: f64, t3136: f64, t365: f64, t863: f64, t872: f64, t882: f64, t891: f64) -> (f64, f64) {
    let t3140 = t3139 * t889;
    let t3143 = -0.310907e-1_f64 * t3080 * t365 + 1.0_f64 * t3083 * t872 + 1.0_f64 * t2252 * t1197 - 2.0_f64 * t2257 * t3088 + 1.0_f64 * t863 * t3103 + 0.32163958997385070134e2_f64 * t2279 * t3107 + t3032 - t3035 - t3037 + t3040 - t3072 - t3076 - 0.19751673498613801407e-1_f64 * t3114 + 0.5848223622634646207e0_f64 * t3116 * t891 + 0.5848223622634646207e0_f64 * t2291 * t1209 - 0.11696447245269292414e1_f64 * t2296 * t3121 + 0.5848223622634646207e0_f64 * t882 * t3136 + 0.17315859105681463759e2_f64 * t2318 * t3140;
    (t3140, t3143)
}
