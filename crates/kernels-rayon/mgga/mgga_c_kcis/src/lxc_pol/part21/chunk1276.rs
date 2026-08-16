//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1276/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1276(t15092: f64, t2189: f64, t3330: f64, t1820: f64, t26950: f64, t1203: f64, t28071: f64, t14654: f64, t283: f64, t990: f64, t26753: f64, t27845: f64, t4994: f64) -> (f64, f64, f64, f64, f64) {
    let t95514 = 2.0_f64 * t3330 * t2189 * t15092;
    let t95517 = 2.0_f64 * t3330 * t26950 * t1820;
    let t95520 = 4.0_f64 * t3330 * t28071 * t1203;
    let t95524 = t14654 * t283 * t990;
    let t95532 = t4994 * t26753 * t27845;
    (t95514, t95517, t95520, t95524, t95532)
}
