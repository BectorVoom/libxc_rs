//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 811/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk811<F: Float>(t486: F, t7052: F, t69: F, t3979: F, t6281: F, t3978: F, t1889: F, t1938: F, t3984: F, t3989: F, t1370: F, t1371: F, t6284: F, t4000: F, t286: F, t7028: F, t1378: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t495 = 0.0 < t486;
    let t7053 = sigma2 * t7052;
    let t7054 = t7053 * t69;
    let t7064 = t3979 * t6281;
    let t7065 = t3978 * t7064;
    let t7068 = t1889 * t1938;
    let t7069 = t3984 * t7068;
    let t7072 = t3989 * t6281;
    let t7073 = t1370 * t7072;
    let t7076 = t1371 * t6284;
    let t7077 = t1370 * t7076;
    let t7080 = t1938 * t1938;
    let t7081 = t4000 * t7080;
    let t7082 = t286 * t7081;
    let t7086 = piecewise3(t495, t7028, -t7028);
    let t7087 = t1378 * t7086;
    (t7053, t7054, t7064, t7065, t7068, t7069, t7072, t7073, t7076, t7077, t7080, t7081, t7082, t7086, t7087)
}
