//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1025/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1025<F: Float>(t10937: F, t11030: F, t11033: F, t17400: F, t17402: F, t17424: F, t17426: F, t23463: F, t23469: F, t23475: F, t23478: F, t23460: F, t23587: F, t23590: F, t23593: F, t23596: F, t23599: F, t23602: F, t23606: F, t23609: F, t23626: F, t23628: F) -> (F, F) {
    let t23679 = -0.13287407407407407408e0 * t10937 - t11030 - t11033 - t17400 + 0.13287407407407407407e0 * t17402 - t17424 + 0.73028148148148148147e-1 * t17426 - 0.33218518518518518518e0 * t23463 + 0.79724444444444444444e0 * t23469 - 0.17938e1 * t23475 - 0.23917333333333333334e1 * t23478;
    let t23703 = -0.73028148148148148146e-1 * t23587 + 0.21908444444444444444e0 * t23590 - 0.98587999999999999998e0 * t23593 - 0.13145066666666666666e1 * t23596 - 0.5477111111111111111e-1 * t23599 - 0.16431333333333333333e0 * t23602 + 0.66437037037037037037e-1 * t23460 + 0.36514074074074074073e-1 * t23606 + 0.10954222222222222222e0 * t23609 + 0.3071625e0 * t23626 + 0.1898925e1 * t23628;
    (t23679, t23703)
}
