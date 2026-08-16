//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1175/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1175(t1170: f64, t14881: f64, t1143: f64, t346: f64, t932: f64, t14051: f64, t143: f64, t1780: f64, t245: f64, t3393: f64, t5155: f64, t330: f64, t4920: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14882 = t1170 * t14881;
    let t14896 = t1143 * t346;
    let t14899 = t1143 * t932;
    let t14902 = t14051 * t143;
    let t14907 = t1780 * t245;
    let t14913 = t3393 * t5155;
    let t14915 = t4920 * t330;
    (t14882, t14896, t14899, t14902, t14907, t14913, t14915)
}
