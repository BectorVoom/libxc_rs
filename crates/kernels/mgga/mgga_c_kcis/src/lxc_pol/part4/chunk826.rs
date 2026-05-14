//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 826/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk826<F: Float>(t3795: F, t4318: F, t5469: F, t5472: F, t5475: F, t5479: F, t1563: F, t2072: F, t1571: F, t2080: F, t3881: F, t4338: F, t4345: F, t5514: F, t5516: F, t5557: F, t5559: F, t5562: F, t5565: F, t5568: F, t5571: F) -> (F, F, F, F) {
    let t6072 = t4318 + 0.57077777777777777777e-2 * t3795 + 0.57077777777777777777e-2 * t5469 - 0.11415555555555555555e-1 * t5472 + 0.34246666666666666666e-1 * t5475 + 0.34246666666666666666e-1 * t5479;
    let t6075 = t2072 * t1563;
    let t6080 = t2080 * t1571;
    let t6097 = -0.17648625e1 * t5514 + 0.3529725e1 * t5516 + t4338 + 0.17215833333333333333e0 * t3795 + 0.17215833333333333333e0 * t5469 - 0.34431666666666666667e0 * t5472 + 0.103295e1 * t5475 + 0.103295e1 * t5479 + 0.31558125e0 * t5557 + 0.6311625e0 * t5559 + t4345 + 0.69463333333333333333e-1 * t3881 + 0.69463333333333333333e-1 * t5562 - 0.34731666666666666667e-1 * t5565 + 0.20839e0 * t5568 + 0.20839e0 * t5571;
    (t6072, t6075, t6080, t6097)
}
