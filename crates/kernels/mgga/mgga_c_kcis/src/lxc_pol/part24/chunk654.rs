//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 654/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk654<F: Float>(t2155: F, t7647: F, t7639: F, t7598: F, t7601: F, t7605: F, t7634: F, t7640: F, t7643: F, t2161: F, t898: F, t2165: F, t906: F) -> (F, F, F) {
    let t7648 = t2155 * t7647;
    let t7650 = t2155 * t7639;
    let t7655 = -F::new(0.69505208333333333333e-3) * t7634 + F::new(0.92754700520833333333e-4) * t7640 + F::new(0.16217881944444444444e-2) * t7643 + F::new(0.69505208333333333333e-3) * t7648 + F::new(0.69505208333333333333e-3) * t7650 - F::new(0.92858888888888888886e-2) * t7598 + F::new(0.69644166666666666665e-2) * t7601 - F::new(0.69644166666666666665e-2) * t7605;
    let t7657 = t2161 * t898;
    let t7660 = t2165 * t906;
    (t7655, t7657, t7660)
}
