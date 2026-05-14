//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 769/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk769<F: Float>(t18547: F, t3293: F, t1045: F, t6317: F, t4642: F, t1728: F, t4670: F, t3255: F, t6594: F, t3269: F, t6330: F, t934: F, t3274: F, t6326: F, t829: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18548 = t3293 * t18547;
    let t18551 = t6317 * t1045;
    let t18552 = t4642 * t18551;
    let t18555 = t1728 * t4670;
    let t18556 = t4642 * t18555;
    let t18559 = t3255 * t6594;
    let t18563 = t3269 * t6330 * t934;
    let t18567 = t3274 * t6330 * t1045;
    let t18570 = t6326 * t829;
    (t18548, t18551, t18552, t18555, t18556, t18559, t18563, t18567, t18570)
}
