//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1411/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1411<F: Float>(t46784: F, t48908: F, t1889: F, t46595: F, t1873: F, t46651: F, t13800: F, t46670: F, t3964: F, t5617: F, t9732: F, t46888: F) -> (F, F, F, F, F, F) {
    let t48909 = t46784 * t48908;
    let t48947 = t46595 * t1889;
    let t49030 = t46651 * t1873;
    let t49087 = t46670 * t13800;
    let t49090 = t3964 * t9732 * t5617;
    let t49105 = t46888 * t48908;
    (t48909, t48947, t49030, t49087, t49090, t49105)
}
