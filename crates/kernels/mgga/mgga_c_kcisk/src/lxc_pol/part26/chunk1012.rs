//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1012/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1012<F: Float>(t26927: F, t6369: F, t21334: F, t21331: F, t6328: F, t21314: F, t469: F, t6333: F, t2270: F, t4229: F, t6323: F, t6373: F, t21029: F, t26404: F, t6321: F, t26889: F, t6322: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27073 = t6369 * t26927;
    let t27074 = t21334 * t27073;
    let t27076 = t21331 * t6328;
    let t27078 = t21314 * t469;
    let t27079 = t27078 * t6333;
    let t27081 = t2270 * t4229;
    let t27082 = t27081 * t6323;
    let t27084 = t21331 * t6373;
    let t27086 = t21029 * t26404;
    let t27087 = t6321 * t27086;
    let t27089 = t6322 * t26889;
    (t27073, t27074, t27076, t27079, t27082, t27084, t27086, t27087, t27089)
}
