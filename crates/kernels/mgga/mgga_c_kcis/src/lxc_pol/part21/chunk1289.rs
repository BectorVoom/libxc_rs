//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1289/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1289<F: Float>(t1020: F, t26671: F, t4806: F, t4548: F, t2842: F, t4556: F, t26753: F, t13217: F, t7718: F, t26748: F, t27826: F, t7703: F, t93342: F, t93344: F, t95721: F, t95727: F, t95730: F, t95736: F) -> (F, F, F, F, F, F) {
    let t95739 = t1020 * t26671 * t4806;
    let t95742 = t1020 * t26671 * t4548;
    let t95745 = t2842 * t26671 * t4556;
    let t95748 = t1020 * t26753 * t4806;
    let t95751 = t1020 * t7718 * t13217;
    let t95753 = F::cast_from(0.92673611111111111112e-3_f64) * t7703 * t95721 + F::cast_from(0.92673611111111111112e-3_f64) * t26748 * t27826 - F::cast_from(0.16581944444444444444e-2_f64) * t95727 - F::cast_from(0.27636574074074074073e-2_f64) * t95730 + F::cast_from(0.20612155671296296296e-4_f64) * t93342 + F::cast_from(0.15445601851851851852e-3_f64) * t93344 + F::cast_from(0.99491666666666666664e-2_f64) * t95736 - F::cast_from(0.58958024691358024689e-2_f64) * t95739 + F::cast_from(0.17687407407407407407e-1_f64) * t95742 - F::cast_from(0.14739506172839506172e-1_f64) * t95745 + F::cast_from(0.22109259259259259258e-2_f64) * t95748 + F::cast_from(0.11054629629629629629e-2_f64) * t95751;
    (t95739, t95742, t95745, t95748, t95751, t95753)
}
