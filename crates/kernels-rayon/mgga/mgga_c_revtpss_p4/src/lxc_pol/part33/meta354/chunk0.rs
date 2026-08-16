//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1372/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1372(t14005: f64, t9816: f64, t2713: f64, t3964: f64, t5617: f64, t5686: f64, t9744: f64, t221: f64, t4019: f64, t5659: f64, t4018: f64, t3989: f64, t5629: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14007 = 0.25410001404642664112e-4_f64 * t9816 * t14005;
    let t14013 = t3964 * t2713 * t5617;
    let t14024 = 7.0_f64 / 24.0_f64 * t9744 * t5686;
    let t14036 = t4019 * t221 * t5659;
    let t14038 = 0.25410001404642664112e-4_f64 * t4018 * t14036;
    let t14040 = 0.40015750243531754508e-1_f64 * t3989 * t5629;
    (t14007, t14013, t14024, t14036, t14038, t14040)
}
