//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1254/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1254<F: Float>(t52446: F, t52452: F, t52591: F, t52593: F, t52596: F, t52601: F, t58415: F, t58418: F, t58421: F, t58424: F, t58428: F, t58431: F, t58435: F, t58754: F, t43503: F, t43508: F, t44329: F, t52687: F, t52689: F, t58758: F, t58761: F, t58763: F, t58765: F, t58770: F, t58774: F, t58776: F, t58778: F, t58780: F) -> (F, F) {
    let t59132 = -0.82156666666666666667e-1 * t58415 - 0.98587999999999999998e0 * t58418 - 0.82156666666666666668e-1 * t58421 + 0.197176e1 * t58424 - 0.85199506172839506175e-1 * t58428 - 0.88582716049382716048e0 * t58431 - 0.29896666666666666667e0 * t58435 + 0.97370864197530864196e-1 * t52591 - 0.43816888888888888888e0 * t52593 + 0.13145066666666666666e1 * t52596 + 0.21908444444444444444e0 * t52601 + 0.79724444444444444444e0 * t52446 - 0.23917333333333333333e1 * t52452 + 0.1898925e1 * t58754;
    let t59147 = 0.46074375e0 * t58758 - 0.3560484375e1 * t58761 - 0.28483875e1 * t58763 + 0.1151859375e0 * t58765 - 0.79724444444444444446e0 * t43503 + 0.15944888888888888889e1 * t43508 - 0.54771111111111111111e0 * t44329 + 0.3071625e0 * t58770 + 0.21908444444444444444e0 * t52687 - 0.13145066666666666666e1 * t52689 - 0.379785e1 * t58774 + 0.614325e0 * t58776 + 0.85451625e1 * t58778 - 0.46074375e0 * t58780;
    (t59132, t59147)
}
