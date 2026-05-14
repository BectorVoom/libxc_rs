//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1136/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1136<F: Float>(t14443: F, t28927: F, t7703: F, t100108: F, t100133: F, t100136: F, t26692: F, t28988: F, t8038: F, t95572: F, t95581: F, t95586: F, t95587: F, t95605: F, t95608: F, t96227: F) -> (F, F, F) {
    let t100986 = t14443 * t28927;
    let t100987 = t7703 * t100986;
    let t100999 = t95572 - 0.58958024691358024688e-2 * t95581 - 0.33163888888888888888e-2 * t100108 + t95586 - 0.22109259259259259259e-2 * t95587 + 0.88437037037037037035e-2 * t100133 + 0.37069444444444444445e-2 * t26692 * t28988 + 0.12356481481481481482e-2 * t96227 * t8038 + 0.27636574074074074073e-2 * t100136 + t95605 + t95608;
    (t100986, t100987, t100999)
}
