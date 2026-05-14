//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 653/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk653<F: Float>(t2155: F, t7647: F, t7639: F, t7598: F, t7601: F, t7605: F, t7634: F, t7640: F, t7643: F, t2161: F, t898: F, t2165: F, t906: F, t7615: F, t7618: F, t7620: F, t7622: F, t7625: F, t7628: F) -> (F, F, F, F) {
    let t7648 = t2155 * t7647;
    let t7650 = t2155 * t7639;
    let t7655 = -0.69505208333333333333e-3 * t7634 + 0.92754700520833333333e-4 * t7640 + 0.16217881944444444444e-2 * t7643 + 0.69505208333333333333e-3 * t7648 + 0.69505208333333333333e-3 * t7650 - 0.92858888888888888886e-2 * t7598 + 0.69644166666666666665e-2 * t7601 - 0.69644166666666666665e-2 * t7605;
    let t7657 = t2161 * t898;
    let t7660 = t2165 * t906;
    let t7669 = 0.1875e0 * t7615 - 0.1875e0 * t7618 - 0.375e0 * t7620 - 0.809375e-1 * t7622 + 0.809375e-1 * t7625 + 0.32375e0 * t7628;
    (t7655, t7657, t7660, t7669)
}
