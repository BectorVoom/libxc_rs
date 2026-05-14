//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1098/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1098<F: Float>(t16721: F, t16775: F, t16779: F, t16783: F, t16787: F, t16886: F, t16889: F, t16893: F, t16897: F, t29127: F, t29128: F, t1020: F, t2706: F, t192: F, t3401: F, t135: F, t16810: F, t16813: F, t16822: F, t20352: F, t20359: F, t20360: F, t20363: F, t2575: F, t2718: F, t29134: F, t29137: F, t7201: F) -> (F, F, F) {
    let t29707 = t29127 - t16886 - t16889 - t29128 - t16893 + t16897 + t16721 - t16775 - t16779 + t16783 - t16787;
    let t29713 = t1020 * t2706;
    let t29718 = t3401 * t192;
    let t29725 = 18.0 * t135 * t2575 * t29718 + 18.0 * t2718 * t3401 * t7201 + t16810 - t16813 - t16822 + t20352 - t20359 + t20360 - t20363 - t29134 - t29137;
    (t29707, t29713, t29725)
}
