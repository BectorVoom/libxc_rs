//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1303/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1303(t26728: f64, t27856: f64, t13376: f64, t4947: f64, t922: f64, t2829: f64, t4781: f64, t26748: f64, t27773: f64, t27958: f64, t2845: f64, t2894: f64, t7703: f64, t8034: f64, t92872: f64, t92908: f64, t92929: f64, t93526: f64, t93709: f64, t9933: f64) -> (f64, f64, f64) {
    let t95963 = t26728 * t27856;
    let t95976 = t4947 * t13376 * t922;
    let t95980 = t4947 * t4781 * t2829;
    let t95983 = 0.14739506172839506172e-2_f64 * t92872 + 0.15445601851851851852e-3_f64 * t93526 - 0.58958024691358024689e-2_f64 * t92908 - 0.22109259259259259258e-2_f64 * t92929 - 0.4946917361111111111e-3_f64 * t93709 * t8034 + 0.61836467013888888888e-4_f64 * t95963 + 0.23168402777777777778e-3_f64 * t7703 * t2894 * t27773 * t2829 + 0.30891203703703703704e-3_f64 * t7703 * t9933 * t27773 * t2845 + 0.46336805555555555556e-3_f64 * t26748 * t27958 + 0.46336805555555555556e-3_f64 * t7703 * t95976 + 0.23168402777777777778e-3_f64 * t7703 * t95980;
    (t95976, t95980, t95983)
}
