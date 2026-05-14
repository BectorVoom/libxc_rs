//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1209/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1209<F: Float>(t240: F, t34832: F, t34834: F, t34836: F, t34837: F, t34838: F, t34840: F, t34842: F, t34845: F, t34848: F, t34851: F, t34889: F, t34922: F, t35042: F, t297: F, t294: F) -> (F, F, F) {
    let t35045 = t34832 - t34834 + t34836 - t34837 - t34838 + t34840 - t34842 - t34845 + t34848 + t34851 - t34889 + t240 * (t34922 + t35042);
    let t35046 = t297 * t35045;
    let t35047 = t294 * t35046;
    (t35045, t35046, t35047)
}
