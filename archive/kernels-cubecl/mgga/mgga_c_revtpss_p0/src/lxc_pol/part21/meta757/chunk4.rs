//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2659/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2659<F: Float>(t13847: F, t13848: F, t4057: F, t9816: F, t13962: F, t9962: F, t13845: F, t48919: F, t5675: F, t9840: F, t1889: F, t46595: F) -> (F, F, F, F, F) {
    let t48929 = t9816 * t13847 * t13848 * t4057;
    let t48937 = t9962 * t13962;
    let t48941 = t13845 * t13847 * t48919 * t5675;
    let t48945 = t13845 * t13847 * t13848 * t9840;
    let t48947 = t46595 * t1889;
    (t48929, t48937, t48941, t48945, t48947)
}
