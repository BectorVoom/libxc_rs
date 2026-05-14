//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 973/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk973<F: Float>(t1783: F, t2889: F, t747: F, t514: F, t7704: F, t1905: F, t11351: F, t1942: F, t452: F, t1971: F, t2919: F, t11529: F, t11535: F, t10959: F, t11066: F, t11073: F, t11076: F, t11532: F, t11539: F, t11542: F, t6323: F, t6337: F, t6467: F, t6635: F, t6651: F, t7437: F, t7438: F, t7442: F) -> (F, F, F, F, F) {
    let t12227 = t1783 * t747 * t2889;
    let t12236 = t514 * t7704;
    let t12237 = t1905 * t12236;
    let t12240 = t11351 * t1942;
    let t12241 = t12240 * t452;
    let t12244 = t2919 * t1971;
    let t12253 = 8.0 * t11529;
    let t12255 = 8.0 * t11535;
    let t12263 = 0.821419393556371 * t11066 + 1.642838787112742 * t10959 + t12253 - 8.0 * t11532 - t12255 + 12.0 * t11539 - 8.0 * t11542 + 0.821419393556371 * t11076 + t7437 + 0.2738064645187903 * t11073 + t7442 - 0.2738064645187903 * t6337 - 0.821419393556371 * t6323 + t6651 + t7438 - t6635 + 0.2738064645187903 * t6467;
    (t12227, t12237, t12241, t12244, t12263)
}
