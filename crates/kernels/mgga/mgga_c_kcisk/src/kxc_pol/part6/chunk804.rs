//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 804/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk804<F: Float>(t1705: F, t8692: F, t4911: F, t8729: F, t1248: F, t4889: F, t8514: F, t10999: F, t8510: F, t8518: F, t45: F, t8740: F) -> (F, F, F, F, F, F) {
    let t23496 = t8692 * t1705;
    let t23528 = t8729 * t4911;
    let t23570 = t1248 * t4889 * t8514;
    let t23606 = t1248 * t10999 * t8510;
    let t23609 = t1248 * t4889 * t8518;
    let t23709 = t45 * t8740;
    (t23496, t23528, t23570, t23606, t23609, t23709)
}
