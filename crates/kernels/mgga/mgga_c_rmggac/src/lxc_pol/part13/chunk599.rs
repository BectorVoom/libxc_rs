//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 599/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk599<F: Float>(t7642: F, t7829: F, t2073: F, t7645: F, t22: F, t874: F, t326: F) -> (F, F, F, F) {
    let t7830 = t7829 * t7642;
    let t7832 = t2073 * t7645;
    let t7834 = t874 * t22;
    let t7835 = t326 * t7834;
    (t7830, t7832, t7834, t7835)
}
