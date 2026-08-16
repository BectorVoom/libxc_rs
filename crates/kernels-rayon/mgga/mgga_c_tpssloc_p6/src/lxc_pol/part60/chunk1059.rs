//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1059/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1059(t128485: f64, t128492: f64, t128498: f64, t128502: f64, t128509: f64, t128511: f64, t128513: f64, t128516: f64, t128523: f64, t128535: f64, t128537: f64, t128539: f64, t128543: f64, t128549: f64, t28030: f64, t29222: f64, t29380: f64, t34170: f64, t7458: f64, t8690: f64, t8835: f64) -> f64 {
    let t130463 = -2.0_f64 * t28030 * t8835 - t29222 * t8690 + 6.0_f64 * t29380 * t8690 - 4.0_f64 * t34170 * t7458 - t128485 - t128492 - t128498 + t128502 - t128509 - t128511 - t128513 - t128516 - t128523 - t128535 - t128537 - t128539 - t128543 + t128549;
    t130463
}
