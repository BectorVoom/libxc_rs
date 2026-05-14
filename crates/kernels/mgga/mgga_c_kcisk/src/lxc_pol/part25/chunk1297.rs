//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1297/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1297<F: Float>(t1636: F, t16991: F, t7242: F, t112585: F, t719: F, t17045: F, t5182: F, t17049: F, t6674: F, t4648: F, t7283: F, t4640: F, t7234: F, t1772: F, t648: F, t64905: F) -> (F, F, F, F, F, F) {
    let t116533 = t7242 * t16991 * t1636;
    let t116536 = t112585 * t719;
    let t116538 = t5182 * t116536 * t17045;
    let t116541 = t6674 * t116536 * t17049;
    let t116544 = t7242 * t7283 * t4648;
    let t116548 = t7234 * t7283 * t4640;
    let t116552 = t64905 * t648 * t1772;
    (t116533, t116538, t116541, t116544, t116548, t116552)
}
