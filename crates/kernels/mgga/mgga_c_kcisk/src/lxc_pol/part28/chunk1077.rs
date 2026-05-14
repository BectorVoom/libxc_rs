//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1077/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1077<F: Float>(t22603: F, t22605: F, t22608: F, t22718: F, t22721: F, t22724: F, t22727: F, t22730: F, t22734: F, t22737: F, t22741: F, t10570: F, t10615: F, t12042: F, t12043: F, t15989: F, t15993: F, t15996: F, t16389: F, t16392: F, t16400: F, t18499: F, t18514: F, t22575: F, t22583: F, t22610: F, t22684: F, t22705: F, t22707: F, t22711: F, t22714: F, t24697: F) -> (F,) {
    let t24719 = 0.13892666666666666667e0 * t22718 - 0.62517e0 * t22721 - 0.83356e0 * t22724 + 0.20839e0 * t22727 - 0.34731666666666666667e-1 * t22730 - 0.3529725e1 * t22605 - 0.17648625e1 * t22608 + 0.6311625e0 * t22734 + 0.31558125e0 * t22737 + 0.264729375e1 * t22603 - 0.157790625e0 * t22741;
    let t24721 = -0.45908888888888888888e0 * t15989 - 0.68863333333333333332e0 * t15996 - 0.11577222222222222222e0 * t10615 - 0.22954444444444444444e0 * t10570 + t18499 - 0.68863333333333333332e0 * t15993 - t12042 - 0.23154444444444444445e0 * t16389 - 0.27785333333333333334e0 * t16392 + 0.20839e0 * t22684 + t24697 - 0.34431666666666666667e0 * t22575 + 0.17215833333333333333e0 * t22583 - 0.13892666666666666667e0 * t22705 + 0.69463333333333333333e-1 * t22707 - t18514 + 0.4630888888888888889e-1 * t16400 - t12043 + 0.3529725e1 * t22610 + 0.6311625e0 * t22711 - 0.46308888888888888889e-1 * t22714 + t24719;
    (t24721,)
}
