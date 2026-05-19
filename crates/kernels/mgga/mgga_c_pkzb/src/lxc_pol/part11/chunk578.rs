//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 578/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk578<F: Float>(t1208: F, t2320: F, t889: F, t1197: F, t1209: F, t2252: F, t2257: F, t2279: F, t2291: F, t2296: F, t2318: F, t3032: F, t3035: F, t3037: F, t3040: F, t3072: F, t3076: F, t3080: F, t3083: F, t3088: F, t3103: F, t3107: F, t3114: F, t3116: F, t3121: F, t3136: F, t365: F, t863: F, t872: F, t882: F, t891: F) -> (F, F, F) {
    let t3139 = t1208 * t2320;
    let t3140 = t3139 * t889;
    let t3143 = -F::new(0.310907e-1) * t3080 * t365 + F::new(1.0) * t3083 * t872 + F::new(1.0) * t2252 * t1197 - F::new(2.0) * t2257 * t3088 + F::new(1.0) * t863 * t3103 + F::cast_from(0.32163958997385070134e2_f64) * t2279 * t3107 + t3032 - t3035 - t3037 + t3040 - t3072 - t3076 - F::cast_from(0.19751673498613801407e-1_f64) * t3114 + F::cast_from(0.5848223622634646207e0_f64) * t3116 * t891 + F::cast_from(0.5848223622634646207e0_f64) * t2291 * t1209 - F::cast_from(0.11696447245269292414e1_f64) * t2296 * t3121 + F::cast_from(0.5848223622634646207e0_f64) * t882 * t3136 + F::cast_from(0.17315859105681463759e2_f64) * t2318 * t3140;
    (t3139, t3140, t3143)
}
