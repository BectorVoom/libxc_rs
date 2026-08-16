//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1080/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1080<F: Float>(t1121: F, t1130: F, t1133: F, t26760: F, t1092: F, t2635: F, t7704: F, t4947: F, t3225: F, t342: F, t3229: F, t303: F) -> (F, F, F, F, F, F, F) {
    let t26761 = t1130 * t1121;
    let t26762 = t26761 * t1133;
    let t26763 = t26760 * t26762;
    let t26764 = t1092 * t26763;
    let t26766 = t7704 * t2635;
    let t26767 = t4947 * t26766;
    let t26772 = t342 * t3225;
    let t26773 = t26772 * t3229;
    let t26774 = t303 * t26773;
    (t26762, t26763, t26764, t26766, t26767, t26773, t26774)
}
