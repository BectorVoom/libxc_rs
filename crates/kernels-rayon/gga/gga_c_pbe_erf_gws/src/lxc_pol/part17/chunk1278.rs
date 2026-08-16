//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1278/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1278(t2408: f64, t3212: f64, t51084: f64, t51540: f64, t51667: f64, t51683: f64, t51688: f64, t53748: f64, t53750: f64, t53751: f64, t53758: f64, t53761: f64, t53768: f64, t53772: f64, t53775: f64, t53779: f64, t53784: f64, t6793: f64, t8629: f64, t9283: f64) -> f64 {
    let t53787 = -t53748 / 384.0_f64 - t53750 + t53751 / 96.0_f64 - t2408 * t9283 * t51084 * t3212 / 12.0_f64 - 7.0_f64 / 576.0_f64 * t51667 + t53758 / 96.0_f64 + t6793 * t53761 / 24.0_f64 + t8629 * t51540 / 48.0_f64 - t53768 / 3072.0_f64 - 7.0_f64 / 48.0_f64 * t51683 - 7.0_f64 / 288.0_f64 * t51688 - t53772 / 96.0_f64 - t53775 / 48.0_f64 - t6793 * t53779 / 12.0_f64 - t6793 * t53784 / 8.0_f64;
    t53787
}
