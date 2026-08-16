//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2338/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2338(t27331: f64, t9231: f64, t2110: f64, t22519: f64, t22531: f64, t22537: f64, t24511: f64, t24526: f64, t26090: f64, t27332: f64, t6492: f64, t7246: f64, t7432: f64, t7435: f64, t7975: f64, t7978: f64, t85514: f64, t85524: f64, t90297: f64, t90337: f64, t90340: f64) -> f64 {
    let t95981 = t9231 * t27331;
    let t95996 = 2.0_f64 / 3.0_f64 * t22519 * t7978 + 5.0_f64 / 3.0_f64 * t24526 * t26090 + 5.0_f64 / 3.0_f64 * t85514 * t7432 + 5.0_f64 / 6.0_f64 * t85524 * t7432 + t7435 * t24511 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t95981 * t6492 + 5.0_f64 / 6.0_f64 * t27332 * t22531 + 2.0_f64 / 3.0_f64 * t22519 * t7975 + t22537 * t7975 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t7246 * t90297 + t90337 * t2110 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t90340 * t2110;
    t95996
}
