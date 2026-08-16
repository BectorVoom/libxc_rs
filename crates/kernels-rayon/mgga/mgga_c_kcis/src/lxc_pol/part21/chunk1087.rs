//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1087/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1087(t26836: f64, t1014: f64, t7732: f64, t3183: f64, t356: f64, t303: f64, t3191: f64, t7727: f64, t1087: f64, t1134: f64, t1086: f64, t7731: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26837 = 0.55273148148148148147e-3_f64 * t26836;
    let t26838 = t1014 * t7732;
    let t26840 = t356 * t3183;
    let t26841 = t303 * t26840;
    let t26843 = t356 * t3191;
    let t26844 = t303 * t26843;
    let t26846 = t1014 * t7727;
    let t26848 = t1087 * t1134;
    let t26849 = t303 * t26848;
    let t26851 = t1086 * t7731;
    (t26837, t26838, t26840, t26841, t26843, t26844, t26846, t26848, t26849, t26851)
}
