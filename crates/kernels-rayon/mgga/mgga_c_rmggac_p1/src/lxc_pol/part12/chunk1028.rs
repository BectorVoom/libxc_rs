//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1028/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1028(t27101: f64, t39044: f64, t39696: f64, t5259: f64, t798: f64, t8946: f64, t4048: f64, t118: f64, t25854: f64, t25877: f64, t27055: f64, t27176: f64, t333: f64, t352: f64, t35980: f64, t35989: f64, t40597: f64, t40721: f64, t41063: f64, t41091: f64, t5148: f64, t5266: f64, t839: f64, t848: f64, t876: f64, t8936: f64) -> (f64, f64, f64) {
    let t41436 = t27101 * t39044;
    let t41438 = t5259 * t39696;
    let t41439 = 0.15965655602485078085e0_f64 * t41438;
    let t41440 = t8946 * t798;
    let t41443 = t8946 * t4048;
    let t41452 = -0.35922725105591425692e0_f64 * t27055 * t8946 * t876 - 0.47896966807455234256e0_f64 * t27176 * t8936 * t839 + 0.11974241701863808564e0_f64 * t118 * t40597 + 0.71845450211182851384e0_f64 * t25854 * t40721 - 0.47896966807455234256e0_f64 * t35980 - 0.79828278012425390426e-1_f64 * t35989 + 0.11974241701863808564e0_f64 * t5266 * t8936 * t848 + 0.5987120850931904282e-1_f64 * t41436 + t41439 + 0.14369090042236570277e1_f64 * t25877 * t41440 + 0.71845450211182851384e0_f64 * t25854 * t41443 - 0.23948483403727617128e0_f64 * t5148 * t41091 * t352 + 0.23948483403727617128e0_f64 * t5266 * t41063 * t333;
    (t41440, t41443, t41452)
}
