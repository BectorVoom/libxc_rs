//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1261/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1261<F: Float>(t1096: F, t14803: F, t14374: F, t5047: F, t7748: F, t14842: F, t28024: F, t3358: F, t4999: F, t2825: F, t5086: F, t1021: F, t14775: F) -> (F, F, F, F, F, F) {
    let t95361 = t1096 * t14803;
    let t95364 = t7748 * t5047 * t14374;
    let t95366 = t28024 * t14842;
    let t95368 = t4999 * t3358;
    let t95370 = t2825 * t5086;
    let t95372 = t1021 * t14775;
    (t95361, t95364, t95366, t95368, t95370, t95372)
}
