//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1316/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1316(t22213: f64, t303: f64, t7931: f64, t102278: f64, t28747: f64, t95024: f64, t1610: f64, t6281: f64, t1615: f64, t6159: f64, t95103: f64, t21854: f64, t4160: f64, t98266: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102563 = t303 * t7931 * t22213;
    let t102568 = t95024 * t102278 * t28747;
    let t102575 = t6281 * t1610;
    let t102580 = t6281 * t1615;
    let t102582 = t6159 * t95103 * t102580;
    let t102586 = t4160 * t98266 * t21854;
    (t102563, t102568, t102575, t102580, t102582, t102586)
}
