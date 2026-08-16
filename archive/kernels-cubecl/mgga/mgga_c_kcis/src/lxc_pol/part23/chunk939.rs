//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 939/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk939<F: Float>(t4281: F, t6034: F, t12564: F, t492: F, t6029: F, t12321: F, t41: F, t15929: F, t5908: F, t4249: F, t6044: F, t15936: F, t6011: F) -> (F, F, F, F, F, F) {
    let t17377 = t4281 * t6034;
    let t17379 = t12564 * t492;
    let t17380 = t17379 * t6029;
    let t17382 = t41 * t12321;
    let t17383 = t17382 * t15929;
    let t17384 = t5908 * t17383;
    let t17386 = t4249 * t6044;
    let t17388 = t6011 * t15936;
    (t17377, t17380, t17383, t17384, t17386, t17388)
}
