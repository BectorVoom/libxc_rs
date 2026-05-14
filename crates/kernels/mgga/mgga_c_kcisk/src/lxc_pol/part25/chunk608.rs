//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 608/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk608<F: Float>(t3290: F, t5486: F, t5006: F, t2020: F, t695: F, t1060: F, t2023: F) -> (F, F, F, F) {
    let t5487 = t5486 * t3290;
    let t5488 = t5006 * t5487;
    let t5491 = t2020 * t695;
    let t5492 = t1060 * t2023;
    (t5487, t5488, t5491, t5492)
}
