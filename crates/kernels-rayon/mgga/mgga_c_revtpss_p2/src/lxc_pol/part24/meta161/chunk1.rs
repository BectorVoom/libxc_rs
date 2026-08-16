//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 809/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk809(t3390: f64, t6442: f64, t3394: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64) -> (f64, f64) {
    let t6443 = t3390 * t6442;
    let t6449 = t3394 - 2.0_f64 / 9.0_f64 * t5044 - 2.0_f64 / 9.0_f64 * t6423 + 2.0_f64 / 3.0_f64 * t6427 + t6431 / 3.0_f64;
    (t6443, t6449)
}
