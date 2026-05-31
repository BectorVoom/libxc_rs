//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 967/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk967<F: Float>(t2890: F, t68: F, t2887: F, t2739: F, t779: F, t655: F, t2888: F, t1843: F, t2889: F, t1885: F, t2946: F, t297: F, t46: F, t768: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7589 = t68 * t2890;
    let t7591 = t2887 * t7589 / F::cast_from(72.0_f64);
    let t7592 = t779 * t2739;
    let t7593 = t7592 * t655;
    let t7594 = t2888 * t7593;
    let t7597 = t2889 * t1843;
    let t7598 = t2888 * t7597;
    let t7601 = t2946 * t1885;
    let t7602 = t2888 * t7601;
    let t7606 = t768 * t297 * t46;
    (t7589, t7591, t7592, t7593, t7594, t7597, t7598, t7601, t7602, t7606)
}
