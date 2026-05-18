//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1373/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1373<F: Float>(t21499: F, t6176: F, t7899: F, t102475: F, t102478: F, t102481: F, t102484: F, t102498: F, t2237: F, t28535: F, t8144: F, t98574: F, t98587: F, t98598: F, t98604: F) -> (F, F) {
    let t103582 = t6176 * t7899 * t21499;
    let t103586 = F::new(0.13901041666666666667e-2) * t8144 * t28535 + t98574 + t98587 - F::new(0.24320185185185185185e-1) * t102475 + F::new(0.1621345679012345679e-1) * t102478 - F::new(0.88437037037037037034e-2) * t102481 + F::new(0.16581944444444444444e-2) * t102484 - t98598 - t98604 + F::new(0.69505208333333333333e-3) * t2237 * t103582 - F::new(0.16581944444444444444e-2) * t102498;
    (t103582, t103586)
}
