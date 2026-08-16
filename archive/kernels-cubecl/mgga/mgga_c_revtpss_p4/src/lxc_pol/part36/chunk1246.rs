//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1246/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1246<F: Float>(t25953: F, t27899: F, t27928: F, t9775: F, t5622: F, t94443: F, t5609: F, t7028: F, t9845: F, t1889: F, t94545: F, t5665: F, t94497: F) -> (F, F, F, F, F, F) {
    let t98104 = t27899 * t25953;
    let t98141 = t9775 * t27928;
    let t98148 = t94443 * t5622;
    let t98161 = t9845 * t7028 * t5609;
    let t98165 = t94545 * t1889;
    let t98174 = t94497 * t5665;
    (t98104, t98141, t98148, t98161, t98165, t98174)
}
