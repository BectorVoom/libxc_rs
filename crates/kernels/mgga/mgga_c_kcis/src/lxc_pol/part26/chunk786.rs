//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 786/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk786<F: Float>(t647: F, t9129: F, t794: F, t8604: F, t103: F, t11: F, t144: F, t148: F, t2477: F, t2542: F, t2546: F, t2555: F, t2561: F, t2565: F, t2569: F, t745: F, t784: F, t791: F, t795: F, t85: F, t8996: F, t9113: F, t9118: F, t9120: F, t9124: F) -> F {
    let t9130 = t9129 * t647;
    let t9144 = t794 * t8604;
    let t9147 = F::new(0.74295e-1) * t9113 * t2561 + F::new(0.4953e-1) * t2546 * t2565 - F::new(0.619125e-2) * t9118 * t9120 - F::new(0.371475e-1) * t9124 * t791 + F::new(0.371475e-1) * t784 * t2569 + F::new(0.619125e-2) * t9130 * t2555 - F::new(0.79593333333333333331e-1) * t85 * t148 * t8996 + F::new(0.5306222222222222222e-1) * t85 * t103 * t745 - F::new(0.15918666666666666666e0) * t85 * t11 * t2477 - F::new(0.1857375e-1) * t2542 * t795 - F::new(0.619125e-2) * t144 * t9144;
    t9147
}
