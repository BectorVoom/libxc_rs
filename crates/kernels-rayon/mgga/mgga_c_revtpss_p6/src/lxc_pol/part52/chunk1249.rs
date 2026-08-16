//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1249/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1249(t128353: f64, t2056: f64, t128355: f64, t34258: f64, t7367: f64, t128493: f64, t128495: f64, t128497: f64, t128499: f64, t128510: f64, t128513: f64, t25805: f64, t28025: f64, t28711: f64, t28760: f64, t33602: f64, t6985: f64, t7374: f64, t7978: f64) -> f64 {
    let t128517 = 2.0_f64 * t128353 * t2056;
    let t128519 = 2.0_f64 * t128355 * t2056;
    let t128521 = 2.0_f64 * t34258 * t7367;
    let t128522 = -2.0_f64 * t25805 * t7978 - 2.0_f64 * t28025 * t7978 - 2.0_f64 * t28711 * t6985 - 2.0_f64 * t28760 * t6985 - 2.0_f64 * t33602 * t7374 - t128493 - t128495 - t128497 - t128499 - t128510 - t128513 - t128517 - t128519 - t128521;
    t128522
}
