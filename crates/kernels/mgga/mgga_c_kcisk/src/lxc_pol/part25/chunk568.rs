//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 568/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk568<F: Float>(t445: F, t5126: F, t5082: F, t1060: F, t696: F, t213: F, t695: F) -> (F, F, F, F) {
    let t5128 = 0.16804375e-4 * t445 * t5126;
    let t5129 = 0.23911438650126355246e-1 * t5082;
    let t5130 = t696 * t1060;
    let t5134 = t213 * t695;
    (t5128, t5129, t5130, t5134)
}
