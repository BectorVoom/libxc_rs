//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 509/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk509<F: Float>(t1882: F, t4652: F, t706: F, t1814: F, t707: F, t1824: F) -> (F, F, F, F) {
    let t4653 = t1882 * t4652;
    let t4654 = t706 * t4653;
    let t4657 = t707 * t1814;
    let t4658 = t1824 * t1824;
    (t4653, t4654, t4657, t4658)
}
