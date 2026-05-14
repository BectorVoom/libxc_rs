//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1216/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1216<F: Float>(t33719: F, t36671: F, t36672: F, t36673: F, t36674: F, t36675: F, t36676: F, t36678: F, t36679: F, t36680: F, t36681: F, t33779: F, t36698: F, t36699: F, t36700: F, t36701: F, t36703: F, t36704: F, t36705: F, t36706: F, t36707: F, t36708: F) -> (F, F) {
    let t38755 = -t36671 - t36672 - t36673 - t36674 + t36675 - t36676 + 0.12650553385416666667e-5 * t33719 + t36678 - t36679 - t36680 + t36681;
    let t38763 = -t36698 - t36699 - t36700 + t36701 - 0.57970906942607043475e-5 * t33779 - t36703 + t36704 + t36705 - t36706 + t36707 + t36708;
    (t38755, t38763)
}
