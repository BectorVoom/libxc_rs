//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2111/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2111<F: Float>(t5609: F, t7028: F, t9845: F, t1889: F, t94545: F, t13846: F, t13877: F, t7021: F, t27932: F, t48525: F, t48141: F, t5665: F, t94497: F) -> (F, F, F, F, F, F) {
    let t98161 = t9845 * t7028 * t5609;
    let t98165 = t94545 * t1889;
    let t98168 = t7021 * t13846 * t13877;
    let t98169 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t98168;
    let t98170 = t27932 * t48525;
    let t98172 = t27932 * t48141;
    let t98174 = t94497 * t5665;
    (t98161, t98165, t98169, t98170, t98172, t98174)
}
