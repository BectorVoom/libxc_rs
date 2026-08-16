//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3102/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3102(t58114: f64, t81439: f64, t81442: f64, t81445: f64, t81448: f64, t81451: f64, t81454: f64, t81457: f64, t81460: f64, t81463: f64, t81466: f64, t81469: f64) -> f64 {
    let t81717 = 0.13892666666666666667e0_f64 * t81439 - 0.10805407407407407407e0_f64 * t81442 - 0.34731666666666666667e-1_f64 * t81445 + 0.62517e0_f64 * t81448 + 0.62517e0_f64 * t81451 + 0.187551e1_f64 * t81454 + 0.250068e1_f64 * t81457 + 0.20839e0_f64 * t81460 - 0.62517e0_f64 * t81463 - 0.125034e1_f64 * t81466 + 0.55570666666666666666e0_f64 * t81469 - t58114;
    t81717
}
