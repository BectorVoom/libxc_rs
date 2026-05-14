//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 987/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk987<F: Float>(t1248: F, t4889: F, t6764: F, t10999: F, t6759: F, t16004: F, t4893: F, t16430: F, t3118: F, t353: F, t16030: F, t342: F, t969: F) -> (F, F, F, F, F, F) {
    let t17423 = t1248 * t4889 * t6764;
    let t17424 = 0.43816888888888888888e0 * t17423;
    let t17426 = t1248 * t10999 * t6759;
    let t17429 = t1248 * t4893 * t16004;
    let t17432 = t353 * t3118 * t16430;
    let t17435 = t342 * t969 * t16030;
    (t17423, t17424, t17426, t17429, t17432, t17435)
}
