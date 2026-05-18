//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 831/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk831<F: Float>(t2551: F, t8735: F, t5109: F, t277: F, t3190: F, t495: F, t360: F, t3052: F) -> (F, F, F, F, F, F) {
    let t8769 = t8735 * t2551;
    let t8770 = t5109 * t8769;
    let t8773 = t277 * t3190;
    let t8774 = t8773 * t495;
    let t8775 = t360 * t8774;
    let t8778 = t277 * t3052;
    (t8769, t8770, t8773, t8774, t8775, t8778)
}
