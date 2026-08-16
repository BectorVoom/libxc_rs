//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1208/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1208(t119639: f64, t119676: f64, t23788: f64, t4255: f64, t118413: f64, t25927: f64, t25365: f64, t118466: f64, t1081: f64, t113111: f64, t113131: f64, t113135: f64, t118399: f64, t118406: f64, t118439: f64, t1877: f64, t22959: f64, t2522: f64, t25372: f64, t25898: f64, t25901: f64, t25905: f64, t25921: f64, t25930: f64, t25934: f64, t25938: f64, t25945: f64, t30753: f64, t30757: f64, t30770: f64, t32886: f64, t6848: f64, t7649: f64, t7656: f64, t8370: f64) -> (f64, f64) {
    let t119677 = t119639 + t119676;
    let t119691 = t23788 * t4255;
    let t119700 = t25927 * t118413;
    let t119713 = t25927 * t25365;
    let t119719 = t23788 * t118466;
    let t119733 = -3.0_f64 / 2.0_f64 * t113131 * t25898 - t1877 * t30757 * t25945 / 2.0_f64 - 3.0_f64 * t118439 * t119691 - 3.0_f64 / 2.0_f64 * t2522 * t8370 * t25901 - 3.0_f64 / 2.0_f64 * t2522 * t8370 * t25905 + 2.0_f64 * t25372 * t119700 + 3.0_f64 / 2.0_f64 * t2522 * t30753 * t7649 + t1877 * t30770 * t25934 + t1877 * t32886 * t1081 / 2.0_f64 + t1877 * t30770 * t25930 + 3.0_f64 * t113135 * t119713 + t118406 - t1877 * t118399 * t6848 / 2.0_f64 - 3.0_f64 * t22959 * t119719 - 3.0_f64 / 2.0_f64 * t113131 * t25921 - t1877 * t30757 * t25930 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2522 * t8370 * t25938 - t1877 * t113111 * t7656 / 2.0_f64;
    (t119677, t119733)
}
