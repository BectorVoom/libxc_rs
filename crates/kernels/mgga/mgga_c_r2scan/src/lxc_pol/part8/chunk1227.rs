//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1227/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1227<F: Float>(t1754: F, t7741: F, t4889: F, t959: F, t5397: F, t5398: F, t956: F, t584: F, t5861: F, t5207: F, t7824: F, t5957: F, t5216: F, t5917: F, t2758: F, t5418: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26405 = t7741 * t1754;
    let t26406 = 0.32530743900905219526e-1 * t26405;
    let t26424 = t4889 * t959;
    let t26427 = t5397 * t956 * t5398;
    let t26430 = t584 * t956 * t5861;
    let t26436 = t7824 * t5207;
    let t26438 = t7824 * t5957;
    let t26442 = t7824 * t5216;
    let t26444 = t7824 * t5917;
    let t26446 = t2758 * t5418;
    (t26406, t26424, t26427, t26430, t26436, t26438, t26442, t26444, t26446)
}
