//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1106/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1106(t1783: f64, t2889: f64, t747: f64, t514: f64, t7704: f64, t1905: f64, t11351: f64, t1942: f64, t452: f64, t1971: f64, t2919: f64, t11529: f64) -> (f64, f64, f64, f64, f64) {
    let t12227 = t1783 * t747 * t2889;
    let t12236 = t514 * t7704;
    let t12237 = t1905 * t12236;
    let t12240 = t11351 * t1942;
    let t12241 = t12240 * t452;
    let t12244 = t2919 * t1971;
    let t12253 = 8.0_f64 * t11529;
    (t12227, t12237, t12241, t12244, t12253)
}
