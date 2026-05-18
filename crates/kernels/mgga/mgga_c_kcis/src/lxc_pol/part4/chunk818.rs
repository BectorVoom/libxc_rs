//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 818/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk818<F: Float>(t2919: F, t2968: F, t3557: F, t3564: F, t4612: F, t4615: F, t4618: F, t4623: F, t4658: F, t4660: F, t4701: F, t4703: F, t4706: F, t4709: F, t4712: F, t4716: F) -> F {
    let t5233 = -F::new(0.17648625e1) * t4658 + F::new(0.3529725e1) * t4660 + t3557 + F::new(0.17215833333333333333e0) * t2919 + F::new(0.17215833333333333333e0) * t4612 - F::new(0.34431666666666666667e0) * t4615 + F::new(0.103295e1) * t4618 - F::new(0.103295e1) * t4623 + F::new(0.31558125e0) * t4701 + F::new(0.6311625e0) * t4703 + t3564 + F::new(0.69463333333333333333e-1) * t2968 + F::new(0.69463333333333333333e-1) * t4706 - F::new(0.34731666666666666667e-1) * t4709 + F::new(0.20839e0) * t4712 - F::new(0.20839e0) * t4716;
    t5233
}
