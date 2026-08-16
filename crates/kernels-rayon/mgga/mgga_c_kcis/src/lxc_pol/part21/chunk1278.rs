//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1278/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1278(t4961: f64, t93426: f64, t95547: f64, t1009: f64, t14407: f64, t2811: f64, t44756: f64, t4566: f64, t26703: f64, t26806: f64, t27832: f64, t44504: f64, t7703: f64, t7704: f64, t93425: f64, t93592: f64, t95524: f64, t95532: f64, t95535: f64, t95537: f64, t95542: f64, t95545: f64, t9924: f64) -> (f64, f64) {
    let t95549 = t93426 * t4961 * t95547;
    let t95552 = t14407 * t1009;
    let t95557 = t44756 * t2811;
    let t95559 = t95557 * t4566 * t95547;
    let t95564 = 0.46336805555555555556e-3_f64 * t27832 * t26703 + 0.61836467013888888889e-4_f64 * t95524 * t26703 - 0.46336805555555555556e-3_f64 * t7703 * t9924 * t7704 * t44504 + 0.66327777777777777776e-2_f64 * t95532 + 0.49555782539766601562e-5_f64 * t95535 * t95537 + 0.55273148148148148147e-3_f64 * t95542 + 0.11054629629629629629e-2_f64 * t95545 - 0.61836467013888888889e-4_f64 * t93425 * t95549 + 0.61782407407407407408e-3_f64 * t93592 * t95552 * t4566 * t26806 + 0.61782407407407407408e-3_f64 * t93592 * t95559 + 0.82448622685185185186e-4_f64 * t93425 * t95559;
    (t95549, t95564)
}
