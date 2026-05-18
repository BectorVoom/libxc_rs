//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1007/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1007<F: Float>(t3255: F, t3777: F, t3757: F, t3789: F, t1444: F, t461: F, t543: F, t3773: F, t1098: F, t3783: F, t3817: F, t1479: F, t3251: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11642 = t3255 * t3777;
    let t11644 = t3255 * t3757;
    let t11646 = t3255 * t3789;
    let t11670 = F::new(1.0) / t461 / t1444;
    let t11671 = t11670 * t543;
    let t11690 = t3255 * t3773;
    let t11708 = t1098 * t3783;
    let t11710 = t1098 * t3817;
    let t11721 = t3251 * t1479;
    (t11642, t11644, t11646, t11670, t11671, t11690, t11708, t11710, t11721)
}
