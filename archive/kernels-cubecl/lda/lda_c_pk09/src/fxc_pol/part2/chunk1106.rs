//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1106/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1106<F: Float>(t1783: F, t2889: F, t747: F, t514: F, t7704: F, t1905: F, t11351: F, t1942: F, t452: F, t1971: F, t2919: F, t11529: F) -> (F, F, F, F, F) {
    let t12227 = t1783 * t747 * t2889;
    let t12236 = t514 * t7704;
    let t12237 = t1905 * t12236;
    let t12240 = t11351 * t1942;
    let t12241 = t12240 * t452;
    let t12244 = t2919 * t1971;
    let t12253 = F::cast_from(8.0_f64) * t11529;
    (t12227, t12237, t12241, t12244, t12253)
}
