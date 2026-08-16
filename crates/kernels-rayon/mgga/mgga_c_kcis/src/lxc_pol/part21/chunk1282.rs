//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1282/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1282(t13097: f64, t27806: f64, t3045: f64, t13132: f64, t13150: f64, t26685: f64, t26692: f64, t26695: f64, t27822: f64, t27826: f64, t27958: f64, t4939: f64, t4947: f64, t7703: f64, t93562: f64, t95537: f64, t95605: f64, t95608: f64, t95621: f64, t95626: f64) -> (f64, f64) {
    let t95629 = t27806 * t13097 * t3045;
    let t95634 = -0.12356481481481481482e-2_f64 * t26692 * t27822 - 0.24712962962962962964e-2_f64 * t26692 * t27826 - 0.16489724537037037037e-3_f64 * t93562 * t27822 + t95605 + t95608 - 0.13901041666666666667e-2_f64 * t7703 * t4947 * t26695 * t13132 - 0.12356481481481481482e-2_f64 * t26692 * t27958 - 0.30891203703703703704e-3_f64 * t7703 * t4939 * t26695 * t13150 - 0.46336805555555555556e-3_f64 * t7703 * t95621 - 0.55273148148148148147e-3_f64 * t95626 - 0.13901041666666666667e-2_f64 * t7703 * t95629 + 0.10203017057291666667e-2_f64 * t26685 * t95537;
    (t95629, t95634)
}
