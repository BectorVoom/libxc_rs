//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 851/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk851<F: Float>(t1885: F, t218: F, t675: F, t1889: F, t1478: F, t154: F, t277: F, t276: F, t2057: F, t739: F, t2045: F, t735: F, t2082: F, t775: F, t2065: F, t771: F) -> (F, F, F, F, F, F, F, F) {
    let t5563 = t218 * t675 * t1885;
    let t5566 = t218 * t675 * t1889;
    let t5589 = t154 * t1478 * t277;
    let t5591 = 5.0 / 1296.0 * t276 * t5589;
    let t5595 = t2057 * t739;
    let t5597 = t735 * t2045;
    let t5607 = t2082 * t775;
    let t5609 = t771 * t2065;
    (t5563, t5566, t5589, t5591, t5595, t5597, t5607, t5609)
}
