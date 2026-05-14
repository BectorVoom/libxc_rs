//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1139/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1139<F: Float>(t1889: F, t3766: F, t3841: F, t1419: F, t5477: F, t16082: F, t5439: F, t16078: F, t16060: F, t5425: F, t11332: F, t3781: F, t11322: F, t3815: F, t1897: F, t1319: F, t5481: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16457 = t3766 * t1889 * t3841;
    let t16461 = t3766 * t5477 * t1419;
    let t16464 = t5439 * t16082;
    let t16467 = t5439 * t16078;
    let t16470 = t5425 * t16060;
    let t16474 = t11332 * t1889 * t3781;
    let t16480 = t11322 * t1889 * t3815;
    let t16483 = t1897 * t3781;
    let t16488 = t5481 * t1319;
    (t16457, t16461, t16464, t16467, t16470, t16474, t16480, t16483, t16488)
}
