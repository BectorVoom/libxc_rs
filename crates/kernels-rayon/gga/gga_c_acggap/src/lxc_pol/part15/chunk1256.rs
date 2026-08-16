//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1256/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1256(t31632: f64, t31644: f64, t35882: f64, t35885: f64, t35890: f64, t35891: f64, t35893: f64, t35894: f64, t35898: f64, t35904: f64, t37777: f64, t37778: f64, t37779: f64, t40295: f64, t40297: f64, t40299: f64, t40301: f64) -> f64 {
    let t42034 = -t35882 / 32.0_f64 - t35885 / 96.0_f64 + t35890 + t35891 + t35893 + t35894 + t35898 - 0.80031500487063509014e-2_f64 * t31632 - t40295 / 32.0_f64 + 0.17149607247227894789e-1_f64 * t40297 - t40299 / 24.0_f64 - t40301 / 24.0_f64 - 0.22675591804667994221e-1_f64 * t31644 - t35904 + t37777 + t37778 + t37779;
    t42034
}
