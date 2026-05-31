//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 778/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk778<F: Float>(t3795: F, t3881: F, t4338: F, t4345: F, t5469: F, t5472: F, t5475: F, t5479: F, t5514: F, t5516: F, t5557: F, t5559: F, t5562: F, t5565: F, t5568: F, t5571: F) -> F {
    let t6097 = -F::cast_from(0.17648625e1_f64) * t5514 + F::cast_from(0.3529725e1_f64) * t5516 + t4338 + F::cast_from(0.17215833333333333333e0_f64) * t3795 + F::cast_from(0.17215833333333333333e0_f64) * t5469 - F::cast_from(0.34431666666666666667e0_f64) * t5472 + F::cast_from(0.103295e1_f64) * t5475 + F::cast_from(0.103295e1_f64) * t5479 + F::cast_from(0.31558125e0_f64) * t5557 + F::cast_from(0.6311625e0_f64) * t5559 + t4345 + F::cast_from(0.69463333333333333333e-1_f64) * t3881 + F::cast_from(0.69463333333333333333e-1_f64) * t5562 - F::cast_from(0.34731666666666666667e-1_f64) * t5565 + F::cast_from(0.20839e0_f64) * t5568 + F::cast_from(0.20839e0_f64) * t5571;
    t6097
}
