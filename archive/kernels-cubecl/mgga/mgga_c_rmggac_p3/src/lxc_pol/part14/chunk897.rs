//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 897/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk897<F: Float>(t118: F, t128: F, t1494: F, t1986: F, t209: F, t7474: F, t1970: F, t1971: F, t236: F, t5615: F, t1243: F, t615: F, t7230: F, t7231: F) -> (F, F, F) {
    let t39513 = t1986 * t118 * t128 * t1494 * t209;
    let t39514 = t7474 * t39513;
    let t39518 = t1970 * t1971 * t236 * t5615;
    let t39523 = t7230 * t7231 * t236 * t615 * t1243;
    (t39514, t39518, t39523)
}
