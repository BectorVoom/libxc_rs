//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 607/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk607<F: Float>(t397: F, t4889: F, t786: F, t782: F, t2015: F, t4998: F, t2013: F, t2024: F, t4419: F, t4597: F) -> (F, F, F, F, F, F, F) {
    let t5477 = t397 * t4889 * t786;
    let t5479 = 0.59969295720591057378e-2 * t782 * t5477;
    let t5480 = t4998 * t2015;
    let t5481 = t2013 * t5480;
    let t5483 = t4419 * t2024;
    let t5484 = t782 * t5483;
    let t5486 = t786 * t4597;
    (t5477, t5479, t5480, t5481, t5483, t5484, t5486)
}
