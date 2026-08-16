//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1260/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1260(t13820: f64, t2409: f64, t3207: f64, t51153: f64, t51156: f64, t51162: f64, t51168: f64, t53487: f64, t53493: f64, t53498: f64, t53503: f64, t53509: f64, t53510: f64, t53513: f64, t53516: f64, t53517: f64, t53520: f64, t8589: f64) -> f64 {
    let t53522 = -7.0_f64 / 1152.0_f64 * t51153 - t53487 / 16.0_f64 - 35.0_f64 / 216.0_f64 * t51156 - t53493 / 768.0_f64 + t53498 / 768.0_f64 - 7.0_f64 / 144.0_f64 * t51162 + 35.0_f64 / 108.0_f64 * t51168 - t53503 - t3207 * t2409 * t8589 * t13820 / 16.0_f64 + t53509 - t53510 / 48.0_f64 + t53513 / 1536.0_f64 + t53516 + t53517 / 24.0_f64 - t53520 / 192.0_f64;
    t53522
}
