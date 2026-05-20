//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2487/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2487<F: Float>(t46784: F, t48908: F, t124: F, t5658: F, t1889: F, t46595: F, t13850: F, t2482: F, t2668: F, t4000: F, t4010: F, t808: F) -> (F, F, F, F, F) {
    let t48909 = t46784 * t48908;
    let t48919 = t124 * t5658;
    let t48947 = t46595 * t1889;
    let t48982 = t2482 * t4000 * t2668 * t13850;
    let t48999 = t808 * t4010;
    (t48909, t48919, t48947, t48982, t48999)
}
