//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 864/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk864(t7052: f64, t69: f64, t3979: f64, t6281: f64, t3978: f64, t1889: f64, t1938: f64, t3984: f64, t3989: f64, t1370: f64, t1371: f64, t6284: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7053 = sigma2 * t7052;
    let t7054 = t7053 * t69;
    let t7064 = t3979 * t6281;
    let t7065 = t3978 * t7064;
    let t7068 = t1889 * t1938;
    let t7069 = t3984 * t7068;
    let t7072 = t3989 * t6281;
    let t7073 = t1370 * t7072;
    let t7076 = t1371 * t6284;
    (t7053, t7054, t7064, t7065, t7068, t7069, t7072, t7073, t7076)
}
