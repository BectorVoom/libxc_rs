//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1012/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1012<F: Float>(t1889: F, t94545: F, t5665: F, t94497: F, t5651: F, t7028: F, t9736: F, t2689: F, t27936: F, t13857: F, t94564: F, t1885: F, t94459: F, t1873: F, t94519: F, t25240: F, t3964: F, t5617: F) -> (F, F, F, F, F, F, F, F) {
    let t98165 = t94545 * t1889;
    let t98174 = t94497 * t5665;
    let t98200 = t9736 * t7028 * t5651;
    let t98218 = t2689 * t27936;
    let t98220 = t94564 * t13857;
    let t98224 = t94459 * t1885;
    let t98260 = t94519 * t1873;
    let t98285 = t3964 * t25240 * t5617;
    (t98165, t98174, t98200, t98218, t98220, t98224, t98260, t98285)
}
