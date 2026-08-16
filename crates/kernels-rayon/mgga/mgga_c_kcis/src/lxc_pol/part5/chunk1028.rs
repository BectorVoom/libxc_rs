//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1028/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1028(t14955: f64, t5135: f64, t1788: f64, t752: f64, t1791: f64, t318: f64, t86: f64, t119: f64, t41: f64, t85: f64, t339: f64, t9368: f64) -> (f64, f64, f64, f64, f64) {
    let t14959 = 0.5895802469135802469e-1_f64 * t14955 * t5135;
    let t14966 = t752 * t1788;
    let t14996 = t86 * t318 * t1791;
    let t15007 = t119 * t41;
    let t15008 = t85 * t15007;
    let t15022 = t9368 * t339;
    (t14959, t14966, t14996, t15008, t15022)
}
