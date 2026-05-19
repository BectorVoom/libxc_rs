//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 564/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk564<F: Float>(t511: F, t7482: F, t1971: F, t1970: F, t2106: F, t261: F) -> (F, F, F) {
    let t7483 = t511 * t7482;
    let t7484 = t1971 * t7483;
    let t7485 = t1970 * t7484;
    let t7486 = F::cast_from(0.25538759935978703638e-4_f64) * t7485;
    let t7487 = t261 * t2106;
    (t7484, t7486, t7487)
}
