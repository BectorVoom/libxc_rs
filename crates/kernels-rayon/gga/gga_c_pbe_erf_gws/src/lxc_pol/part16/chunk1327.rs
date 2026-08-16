//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1327/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1327(t53729: f64, t51651: f64, t51667: f64, t51683: f64, t51688: f64, t52131: f64, t52432: f64, t53734: f64, t53736: f64, t53742: f64, t53748: f64, t53751: f64, t53758: f64, t53768: f64, t53772: f64, t8793: f64) -> f64 {
    let t55351 = 7.0_f64 / 576.0_f64 * t53729;
    let t55367 = -t55351 + t53734 / 24.0_f64 - t53736 / 24.0_f64 + t53742 / 768.0_f64 - t53748 / 192.0_f64 - 35.0_f64 / 54.0_f64 * t51651 + t53751 / 48.0_f64 - 7.0_f64 / 288.0_f64 * t51667 + t53758 / 48.0_f64 - 7.0_f64 / 72.0_f64 * t52432 + t8793 * t52131 / 48.0_f64 - t53768 / 1536.0_f64 - 7.0_f64 / 24.0_f64 * t51683 - 7.0_f64 / 144.0_f64 * t51688 - t53772 / 48.0_f64;
    t55367
}
