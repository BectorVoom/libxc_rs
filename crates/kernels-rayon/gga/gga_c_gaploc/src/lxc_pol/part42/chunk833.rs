//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 833/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk833(t44725: f64, t42942: f64, t13630: f64, t1841: f64, t2536: f64, t734: f64, t42953: f64, t2576: f64, t35435: f64, t161: f64, t36610: f64, t42963: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44726 = 0.64087718584518535698e-3_f64 * t44725;
    let t44731 = 0.1281754371690370714e-2_f64 * t42942;
    let t44735 = 0.85450291446024714263e-3_f64 * t1841 * t2536 * t13630 * t734;
    let t44740 = 0.17090058289204942853e-2_f64 * t42953;
    let t44744 = 0.59815204012217299984e-2_f64 * t1841 * t35435 * t2576;
    let t44745 = t36610 * t161;
    let t44748 = 0.25635087433807414279e-2_f64 * t1841 * t44745 * t2576;
    let t44751 = 0.15381052460284448568e-1_f64 * t42963;
    (t44726, t44731, t44735, t44740, t44744, t44748, t44751)
}
