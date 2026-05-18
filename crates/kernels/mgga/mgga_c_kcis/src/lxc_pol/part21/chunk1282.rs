//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1282/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1282<F: Float>(t13097: F, t27806: F, t3045: F, t13132: F, t13150: F, t26685: F, t26692: F, t26695: F, t27822: F, t27826: F, t27958: F, t4939: F, t4947: F, t7703: F, t93562: F, t95537: F, t95605: F, t95608: F, t95621: F, t95626: F) -> (F, F) {
    let t95629 = t27806 * t13097 * t3045;
    let t95634 = -F::new(0.12356481481481481482e-2) * t26692 * t27822 - F::new(0.24712962962962962964e-2) * t26692 * t27826 - F::new(0.16489724537037037037e-3) * t93562 * t27822 + t95605 + t95608 - F::new(0.13901041666666666667e-2) * t7703 * t4947 * t26695 * t13132 - F::new(0.12356481481481481482e-2) * t26692 * t27958 - F::new(0.30891203703703703704e-3) * t7703 * t4939 * t26695 * t13150 - F::new(0.46336805555555555556e-3) * t7703 * t95621 - F::new(0.55273148148148148147e-3) * t95626 - F::new(0.13901041666666666667e-2) * t7703 * t95629 + F::new(0.10203017057291666667e-2) * t26685 * t95537;
    (t95629, t95634)
}
