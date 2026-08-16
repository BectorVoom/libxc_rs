//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1280/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1280<F: Float>(t27765: F, t2861: F, t27769: F, t13097: F, t26686: F, t93427: F, t26688: F, t26692: F, t27812: F, t27816: F, t27832: F, t8038: F, t93412: F, t95524: F, t95566: F, t95569: F, t95572: F, t95579: F, t95581: F) -> (F, F, F, F) {
    let t95585 = t2861 * t27765;
    let t95586 = F::cast_from(0.66327777777777777776e-2_f64) * t95585;
    let t95587 = t2861 * t27769;
    let t95590 = t26686 * t13097 * t93427;
    let t95595 = F::cast_from(0.22109259259259259258e-2_f64) * t95566 + F::cast_from(0.99491666666666666664e-2_f64) * t95569 + t95572 - F::cast_from(0.13901041666666666667e-2_f64) * t27832 * t26688 - F::cast_from(0.18550940104166666667e-3_f64) * t95524 * t26688 + F::cast_from(0.73697530864197530862e-3_f64) * t95579 - F::cast_from(0.58958024691358024689e-2_f64) * t95581 - F::cast_from(0.23168402777777777778e-3_f64) * t93412 * t8038 + t95586 - F::cast_from(0.22109259259259259258e-2_f64) * t95587 + F::cast_from(0.37134344353515625e-4_f64) * t27812 * t95590 - F::cast_from(0.12356481481481481482e-2_f64) * t26692 * t27816;
    (t95585, t95587, t95590, t95595)
}
