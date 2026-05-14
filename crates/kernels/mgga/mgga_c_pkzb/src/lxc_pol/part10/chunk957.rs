//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 957/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk957<F: Float>(t655: F, t7592: F, t2888: F, t1843: F, t2889: F, t1885: F, t2946: F, t297: F, t46: F, t768: F, t2942: F, t2883: F, t735: F, t2887: F, t2891: F, t2948: F, t5591: F, t5595: F, t5597: F, t5607: F, t5609: F, t5614: F, t5620: F, t5630: F, t757: F, t7578: F, t7582: F, t7585: F, t7586: F, t7591: F) -> (F, F, F, F, F, F) {
    let t7593 = t7592 * t655;
    let t7594 = t2888 * t7593;
    let t7597 = t2889 * t1843;
    let t7598 = t2888 * t7597;
    let t7601 = t2946 * t1885;
    let t7602 = t2888 * t7601;
    let t7606 = t768 * t297 * t46;
    let t7607 = t2942 * t7606;
    let t7617 = t735 * t2883 / 54.0;
    let t7618 = 0.21437009059034868486e-3 * t757 * t7578 - 0.47637797908966374413e-4 * t7582 - t7585 - t7586 * t2891 / 9.0 + t7591 + t2887 * t7594 / 24.0 + t2887 * t7598 / 48.0 - t2887 * t7602 / 16.0 - 0.13719685797782315831e-1 * t7607 * t2948 + t5591 + 11.0 / 324.0 * t5595 + t5597 / 81.0 + 0.48272968547752592739e-2 * t5607 + 0.5081365110289746604e-3 * t5609 + t5614 + 0.30488190661738479624e-2 * t5620 + 0.85748036236139473944e-3 * t5630 + t7617;
    (t7593, t7597, t7601, t7607, t7617, t7618)
}
