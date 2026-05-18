//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 502/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk502<F: Float>(t1398: F, t4142: F, t1397: F, t3738: F, t1394: F, t1396: F, t3805: F, t1395: F, t1017: F, t3751: F, t86: F) -> (F, F, F, F, F, F, F) {
    let t4143 = t4142 * t1398;
    let t4145 = t3738 * t1397;
    let t4146 = t1394 * t4145;
    let t4148 = t1396 * t3805;
    let t4149 = t1395 * t4148;
    let t4150 = t1394 * t4149;
    let t4153 = t86 * t1017 * t3751;
    (t4143, t4145, t4146, t4148, t4149, t4150, t4153)
}
