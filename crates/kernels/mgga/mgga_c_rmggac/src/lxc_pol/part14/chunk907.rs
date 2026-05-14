//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 907/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk907<F: Float>(t4048: F, t8975: F, t39372: F, t5148: F, t40864: F, t4669: F, t118: F, t25820: F, t27101: F, t321: F, t40589: F, t41063: F, t41535: F, t41537: F, t41538: F, t41540: F, t41542: F, t41544: F, t41550: F, t41551: F) -> (F, F) {
    let t41554 = t8975 * t4048;
    let t41560 = t5148 * t39372;
    let t41562 = t4669 * t40864;
    let t41564 = -t41535 - t41537 + 0.71845450211182851384e0 * t41538 + 0.17961362552795712846e0 * t41540 - 0.35922725105591425692e0 * t41542 + 0.17961362552795712846e0 * t41544 - 0.39914139006212695214e-1 * t118 * t40589 - t41550 - 0.71845450211182851384e0 * t25820 * t41551 - 0.47896966807455234256e0 * t27101 * t41554 - 0.23948483403727617128e0 * t5148 * t41063 * t321 + 0.2993560425465952141e-1 * t41560 + 0.44903406381989282115e-1 * t41562;
    (t41554, t41564)
}
