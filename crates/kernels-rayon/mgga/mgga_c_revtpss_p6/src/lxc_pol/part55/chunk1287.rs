//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1287/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1287(t128483: f64, t128485: f64, t128487: f64, t128490: f64, t128493: f64, t128495: f64, t128497: f64, t128499: f64, t128510: f64, t128513: f64, t129377: f64, t27126: f64, t28696: f64, t28929: f64, t33306: f64, t7586: f64, t7732: f64, t8892: f64) -> f64 {
    let t130984 = 6.0_f64 * t129377 * t28929 - 2.0_f64 * t27126 * t8892 - 2.0_f64 * t28696 * t7586 - 2.0_f64 * t33306 * t7732 - t128483 - t128485 - t128487 - t128490 - t128493 - t128495 - t128497 - t128499 - t128510 - t128513;
    t130984
}
