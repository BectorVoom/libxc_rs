//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1280/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1280(t27765: f64, t2861: f64, t27769: f64, t13097: f64, t26686: f64, t93427: f64, t26688: f64, t26692: f64, t27812: f64, t27816: f64, t27832: f64, t8038: f64, t93412: f64, t95524: f64, t95566: f64, t95569: f64, t95572: f64, t95579: f64, t95581: f64) -> (f64, f64, f64, f64) {
    let t95585 = t2861 * t27765;
    let t95586 = 0.66327777777777777776e-2_f64 * t95585;
    let t95587 = t2861 * t27769;
    let t95590 = t26686 * t13097 * t93427;
    let t95595 = 0.22109259259259259258e-2_f64 * t95566 + 0.99491666666666666664e-2_f64 * t95569 + t95572 - 0.13901041666666666667e-2_f64 * t27832 * t26688 - 0.18550940104166666667e-3_f64 * t95524 * t26688 + 0.73697530864197530862e-3_f64 * t95579 - 0.58958024691358024689e-2_f64 * t95581 - 0.23168402777777777778e-3_f64 * t93412 * t8038 + t95586 - 0.22109259259259259258e-2_f64 * t95587 + 0.37134344353515625e-4_f64 * t27812 * t95590 - 0.12356481481481481482e-2_f64 * t26692 * t27816;
    (t95585, t95587, t95590, t95595)
}
