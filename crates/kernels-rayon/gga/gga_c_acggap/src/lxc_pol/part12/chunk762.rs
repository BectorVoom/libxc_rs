//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 762/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk762(t7387: f64, t7390: f64, t7394: f64, t7396: f64, t7398: f64, t7403: f64, t7405: f64, t7407: f64, t7409: f64, t7411: f64, t7416: f64, t7420: f64, t7424: f64, t7429: f64, t7434: f64, t7438: f64, t7441: f64, t7445: f64, t7448: f64, t7453: f64) -> f64 {
    let t8169 = -t7387 / 48.0_f64 - 0.305625e-1_f64 * t7390 + t7394 / 96.0_f64 + 0.5603125e-1_f64 * t7396 + t7398 / 24.0_f64 + t7403 / 16.0_f64 + 7.0_f64 / 72.0_f64 * t7405 - t7407 / 12.0_f64 - t7409 / 24.0_f64 - t7411 / 24.0_f64 - 0.62896184579208304138e-3_f64 * t7416 + 0.31448092289604152069e-3_f64 * t7420 - 0.21437009059034868486e-3_f64 * t7424 - 0.18868855373762491241e-2_f64 * t7429 - 0.37737710747524982482e-2_f64 * t7434 + t7438 / 12.0_f64 - 0.1120625e0_f64 * t7441 - 0.4584375e-1_f64 * t7445 - 0.16809375e0_f64 * t7448 - 0.916875e-1_f64 * t7453;
    t8169
}
