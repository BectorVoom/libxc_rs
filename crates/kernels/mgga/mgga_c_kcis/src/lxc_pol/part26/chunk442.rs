//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 442/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk442<F: Float>(t3251: F, t546: F, t1474: F, t3255: F, t1098: F, t1479: F, t1484: F, t461: F, t531: F) -> (F, F, F, F, F) {
    let t3743 = F::cast_from(0.21901432222222222222e-3_f64) * t3251 * t546;
    let t3744 = t3255 * t1474;
    let t3746 = t1098 * t1479;
    let t3748 = t1098 * t1484;
    let t3750 = t461 * t531;
    let t3751 = F::cast_from(1.0_f64) / t3750;
    (t3743, t3744, t3746, t3748, t3751)
}
