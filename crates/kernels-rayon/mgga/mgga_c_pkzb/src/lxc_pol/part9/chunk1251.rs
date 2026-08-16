//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1251/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1251(t2099: f64, t2945: f64, t7788: f64, t7792: f64, t7797: f64, t2079: f64, t297: f64, t46: f64, t18110: f64, t18114: f64, t18121: f64, t18123: f64, t2003: f64, t2942: f64, t2946: f64, t2948: f64, t5537: f64, t655: f64, t7350: f64, t758: f64, t7607: f64, t7789: f64, t7793: f64, t7798: f64) -> f64 {
    let t21874 = t2945 * t2099 * t7788;
    let t21877 = t2945 * t2099 * t7792;
    let t21882 = t2945 * t2099 * t7797;
    let t21902 = t2079 * t297 * t46;
    let t21906 = 0.51448821741683684367e-2_f64 * t21874 + 0.25724410870841842184e-2_f64 * t21877 + 0.82318114786693894987e-1_f64 * t7607 * t7798 - 0.10289764348336736874e-1_f64 * t21882 - 0.3811023832717309953e-3_f64 * t18110 + 0.28582678745379824648e-3_f64 * t18114 + 0.12862205435420921092e-2_f64 * t2945 * t758 * t2946 * t5537 + 0.38586616306262763276e-2_f64 * t2945 * t758 * t2003 * t7350 * t655 - 0.41159057393346947493e-1_f64 * t7607 * t7789 - 0.20579528696673473747e-1_f64 * t7607 * t7793 - 77.0_f64 / 486.0_f64 * t18121 - 11.0_f64 / 162.0_f64 * t18123 + 0.13033701507893200039e0_f64 * t2942 * t21902 * t2948;
    t21906
}
