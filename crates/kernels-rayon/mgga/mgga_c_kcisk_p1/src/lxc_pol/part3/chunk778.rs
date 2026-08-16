//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 778/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk778(t10501: f64, t1992: f64, t772: f64, t5520: f64, t9726: f64, t1961: f64, t5372: f64, t10568: f64, t10570: f64, t10572: f64, t10574: f64, t10576: f64, t10579: f64, t10582: f64, t10587: f64, t10590: f64, t10595: f64, t10598: f64) -> (f64, f64, f64, f64, f64) {
    let t11983 = 0.51588271604938271604e-3_f64 * t10501;
    let t11984 = t1992 * t1992;
    let t11985 = 1.0_f64 / t11984;
    let t11986 = t772 * t11985;
    let t11991 = t9726 * t5520;
    let t11999 = t1961 * t5372;
    let t12002 = 0.53272592592592592592e-1_f64 * t10568;
    let t12013 = -t12002 - 0.2283111111111111111e-1_f64 * t10570 + 0.11415555555555555555e-1_f64 * t10572 - 0.34246666666666666665e-1_f64 * t10574 + 0.17123333333333333333e-1_f64 * t10576 - 0.19025925925925925925e-1_f64 * t10579 + 0.68493333333333333331e-1_f64 * t10582 - 0.34246666666666666665e-1_f64 * t10587 - 0.10274e0_f64 * t10590 + 0.10274e0_f64 * t10595 - 0.17123333333333333333e-1_f64 * t10598;
    (t11983, t11986, t11991, t11999, t12013)
}
