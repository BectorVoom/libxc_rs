//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1261/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1261<F: Float>(t52446: F, t52452: F, t52591: F, t52593: F, t52596: F, t52601: F, t58415: F, t58418: F, t58421: F, t58424: F, t58428: F, t58431: F, t58435: F, t58754: F, t43503: F, t43508: F, t44329: F, t52687: F, t52689: F, t58758: F, t58761: F, t58763: F, t58765: F, t58770: F, t58774: F, t58776: F, t58778: F, t58780: F) -> (F, F) {
    let t59310 = -0.104195e0 * t58415 - 0.125034e1 * t58418 - 0.104195e0 * t58421 + 0.250068e1 * t58424 - 0.10805407407407407407e0 * t58428 - 0.15302962962962962963e1 * t58431 - 0.516475e0 * t58435 + 0.12349037037037037037e0 * t52591 - 0.55570666666666666668e0 * t52593 + 0.166712e1 * t52596 + 0.27785333333333333333e0 * t52601 + 0.13772666666666666667e1 * t52446 - 0.41318e1 * t52452 + 0.3529725e1 * t58754;
    let t59325 = 0.94674375e0 * t58758 - 0.6618234375e1 * t58761 - 0.52945875e1 * t58763 + 0.2366859375e0 * t58765 - 0.13772666666666666666e1 * t43503 + 0.27545333333333333333e1 * t43508 - 0.69463333333333333334e0 * t44329 + 0.6311625e0 * t58770 + 0.27785333333333333333e0 * t52687 - 0.166712e1 * t52689 - 0.705945e1 * t58774 + 0.1262325e1 * t58776 + 0.158837625e2 * t58778 - 0.94674375e0 * t58780;
    (t59310, t59325)
}
