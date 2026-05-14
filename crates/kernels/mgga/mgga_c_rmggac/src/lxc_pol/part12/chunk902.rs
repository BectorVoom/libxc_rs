//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 902/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk902<F: Float>(t27101: F, t39044: F, t39696: F, t5259: F, t798: F, t8946: F, t4048: F, t118: F, t25854: F, t25877: F, t27055: F, t27176: F, t333: F, t352: F, t35980: F, t35989: F, t40597: F, t40721: F, t41063: F, t41091: F, t5148: F, t5266: F, t839: F, t848: F, t876: F, t8936: F) -> (F, F, F) {
    let t41436 = t27101 * t39044;
    let t41438 = t5259 * t39696;
    let t41439 = 0.15965655602485078085e0 * t41438;
    let t41440 = t8946 * t798;
    let t41443 = t8946 * t4048;
    let t41452 = -0.35922725105591425692e0 * t27055 * t8946 * t876 - 0.47896966807455234256e0 * t27176 * t8936 * t839 + 0.11974241701863808564e0 * t118 * t40597 + 0.71845450211182851384e0 * t25854 * t40721 - 0.47896966807455234256e0 * t35980 - 0.79828278012425390426e-1 * t35989 + 0.11974241701863808564e0 * t5266 * t8936 * t848 + 0.5987120850931904282e-1 * t41436 + t41439 + 0.14369090042236570277e1 * t25877 * t41440 + 0.71845450211182851384e0 * t25854 * t41443 - 0.23948483403727617128e0 * t5148 * t41091 * t352 + 0.23948483403727617128e0 * t5266 * t41063 * t333;
    (t41440, t41443, t41452)
}
