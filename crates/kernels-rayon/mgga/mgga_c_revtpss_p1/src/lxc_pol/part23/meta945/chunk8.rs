//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3112/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3112(t56176: f64, t81439: f64, t81442: f64, t81445: f64, t81448: f64, t81451: f64, t81454: f64, t81457: f64, t81460: f64, t81463: f64, t81466: f64, t81469: f64) -> f64 {
    let t81944 = 0.11038e0_f64 * t81439 - 0.8585111111111111111e-1_f64 * t81442 - 0.27595e-1_f64 * t81445 + 0.49671e0_f64 * t81448 + 0.49671e0_f64 * t81451 + 0.149013e1_f64 * t81454 + 0.198684e1_f64 * t81457 + 0.16557e0_f64 * t81460 - 0.49671e0_f64 * t81463 - 0.99342e0_f64 * t81466 + 0.44152e0_f64 * t81469 - 0.26837777777777777779e0_f64 * t56176;
    t81944
}
