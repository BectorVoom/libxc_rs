//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 872/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk872<F: Float>(t44655: F, t7474: F, t1494: F, t1970: F, t209: F, t236: F, t3352: F, t551: F, t1971: F, t615: F, t7453: F, t10072: F, t7255: F) -> (F, F, F, F) {
    let t44656 = t7474 * t44655;
    let t44662 = t1970 * t3352 * t236 * t551 * t1494 * t209;
    let t44668 = t7453 * t1971 * t236 * t615 * t1494 * t209;
    let t44670 = t7255 * t10072;
    (t44656, t44662, t44668, t44670)
}
