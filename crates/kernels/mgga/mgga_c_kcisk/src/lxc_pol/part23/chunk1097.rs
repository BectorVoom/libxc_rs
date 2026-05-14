//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1097/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1097<F: Float>(t15087: F, t15094: F, t20928: F, t20930: F, t20933: F, t20936: F, t20939: F, t21342: F, t21345: F, t21348: F, t22047: F, t4530: F, t4536: F, t555: F, t6607: F, t6638: F) -> (F,) {
    let t22051 = 4.0 * t15087 * t6607 - 6.0 * t15094 * t21348 + 2.0 * t21345 * t4536 + t22047 * t555 - 2.0 * t4530 * t6638 - t20928 + t20930 + t20933 - t20936 - t20939 + t21342;
    (t22051,)
}
