//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1267/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1267(t7968: f64, t99059: f64, t98794: f64, t98863: f64, t18221: f64, t28843: f64, t7978: f64, t28793: f64, t7974: f64, t98887: f64, t98918: f64, t27601: f64, t28714: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t99610 = 0.92754700520833333333e-4_f64 * t7968 * t99059;
    let t99615 = 0.10317654320987654321e-2_f64 * t98794;
    let t99630 = 0.23214722222222222222e-2_f64 * t98863;
    let t99639 = t7978 * t18221 * t28843;
    let t99644 = 0.61782407407407407408e-3_f64 * t28793 * t7974;
    let t99646 = 0.23214722222222222222e-2_f64 * t98887;
    let t99667 = 0.15476481481481481481e-2_f64 * t98918;
    let t99671 = 0.23168402777777777778e-3_f64 * t28714 * t27601;
    (t99610, t99615, t99630, t99639, t99644, t99646, t99667, t99671)
}
