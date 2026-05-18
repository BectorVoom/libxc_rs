//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1287/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1287<F: Float>(t11322: F, t1889: F, t3815: F, t1897: F, t3781: F, t1319: F, t5481: F, t3809: F, t1958: F, t3820: F, t1317: F, t5523: F) -> (F, F, F, F, F, F) {
    let t16480 = t11322 * t1889 * t3815;
    let t16483 = t1897 * t3781;
    let t16488 = t5481 * t1319;
    let t16491 = t1897 * t3809;
    let t16500 = t3820 * t1958;
    let t16503 = t1317 * t5523;
    (t16480, t16483, t16488, t16491, t16500, t16503)
}
