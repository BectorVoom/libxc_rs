//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 659/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk659<F: Float>(t1498: F, t3738: F, t1464: F, t3251: F, t546: F, t1474: F, t3255: F, t1098: F, t1479: F, t1484: F, t461: F, t531: F) -> (F, F, F, F, F, F, F) {
    let t3739 = t3738 * t1498;
    let t3740 = t1464 * t3739;
    let t3743 = F::new(0.21901432222222222222e-3) * t3251 * t546;
    let t3744 = t3255 * t1474;
    let t3746 = t1098 * t1479;
    let t3748 = t1098 * t1484;
    let t3750 = t461 * t531;
    (t3739, t3740, t3743, t3744, t3746, t3748, t3750)
}
