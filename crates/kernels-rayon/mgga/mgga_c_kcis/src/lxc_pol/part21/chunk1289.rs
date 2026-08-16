//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1289/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1289(t1020: f64, t26671: f64, t4806: f64, t4548: f64, t2842: f64, t4556: f64, t26753: f64, t13217: f64, t7718: f64, t26748: f64, t27826: f64, t7703: f64, t93342: f64, t93344: f64, t95721: f64, t95727: f64, t95730: f64, t95736: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95739 = t1020 * t26671 * t4806;
    let t95742 = t1020 * t26671 * t4548;
    let t95745 = t2842 * t26671 * t4556;
    let t95748 = t1020 * t26753 * t4806;
    let t95751 = t1020 * t7718 * t13217;
    let t95753 = 0.92673611111111111112e-3_f64 * t7703 * t95721 + 0.92673611111111111112e-3_f64 * t26748 * t27826 - 0.16581944444444444444e-2_f64 * t95727 - 0.27636574074074074073e-2_f64 * t95730 + 0.20612155671296296296e-4_f64 * t93342 + 0.15445601851851851852e-3_f64 * t93344 + 0.99491666666666666664e-2_f64 * t95736 - 0.58958024691358024689e-2_f64 * t95739 + 0.17687407407407407407e-1_f64 * t95742 - 0.14739506172839506172e-1_f64 * t95745 + 0.22109259259259259258e-2_f64 * t95748 + 0.11054629629629629629e-2_f64 * t95751;
    (t95739, t95742, t95745, t95748, t95751, t95753)
}
