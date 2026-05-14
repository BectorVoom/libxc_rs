//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1127/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1127<F: Float>(t26688: F, t26692: F, t27812: F, t27816: F, t27832: F, t8038: F, t93412: F, t95524: F, t95566: F, t95569: F, t95572: F, t95579: F, t95581: F, t95586: F, t95587: F, t95590: F) -> (F,) {
    let t95595 = 0.22109259259259259258e-2 * t95566 + 0.99491666666666666664e-2 * t95569 + t95572 - 0.13901041666666666667e-2 * t27832 * t26688 - 0.18550940104166666667e-3 * t95524 * t26688 + 0.73697530864197530862e-3 * t95579 - 0.58958024691358024689e-2 * t95581 - 0.23168402777777777778e-3 * t93412 * t8038 + t95586 - 0.22109259259259259258e-2 * t95587 + 0.37134344353515625e-4 * t27812 * t95590 - 0.12356481481481481482e-2 * t26692 * t27816;
    (t95595,)
}
