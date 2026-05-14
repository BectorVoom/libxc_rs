//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 804/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk804<F: Float>(t20839: F, t20851: F, t44: F, t1489: F, t4163: F, t6284: F, t4162: F, t4160: F, t1497: F, t4171: F, t4170: F, t833: F) -> (F, F, F, F, F, F) {
    let t20853 = (t20839 + t20851) * t44;
    let t20873 = t4163 * t6284 * t1489;
    let t20874 = t4162 * t20873;
    let t20875 = t4160 * t20874;
    let t20878 = t4171 * t6284 * t1497;
    let t20879 = t4170 * t20878;
    let t20880 = t4160 * t20879;
    let t20882 = t6284 * t833;
    (t20853, t20873, t20875, t20878, t20880, t20882)
}
