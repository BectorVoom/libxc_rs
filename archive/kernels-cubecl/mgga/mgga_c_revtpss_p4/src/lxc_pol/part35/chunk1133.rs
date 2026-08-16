//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1133/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1133<F: Float>(t5622: F, t94443: F, t5609: F, t7028: F, t9845: F, t1889: F, t94545: F, t5665: F, t94497: F, t5651: F, t9736: F, t2689: F, t27936: F) -> (F, F, F, F, F, F) {
    let t98148 = t94443 * t5622;
    let t98161 = t9845 * t7028 * t5609;
    let t98165 = t94545 * t1889;
    let t98174 = t94497 * t5665;
    let t98200 = t9736 * t7028 * t5651;
    let t98218 = t2689 * t27936;
    (t98148, t98161, t98165, t98174, t98200, t98218)
}
