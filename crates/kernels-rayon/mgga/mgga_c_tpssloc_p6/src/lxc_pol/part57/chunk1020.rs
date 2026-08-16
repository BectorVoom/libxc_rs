//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1020/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1020(t33214: f64, t7796: f64, t28030: f64, t8533: f64, t128474: f64, t128475: f64, t128477: f64, t128482: f64, t128485: f64, t128492: f64, t128498: f64, t128502: f64, t128509: f64, t1774: f64, t2040: f64, t28852: f64, t28855: f64, t31532: f64, t33579: f64, t5494: f64, t652: f64, t7042: f64, t7670: f64, t7801: f64, t8329: f64, t96686: f64) -> f64 {
    let t128511 = 4.0_f64 * t33214 * t7796;
    let t128513 = 2.0_f64 * t28030 * t8533;
    let t128514 = -4.0_f64 * t652 * t7670 * t7801 - 2.0_f64 * t1774 * t33579 - 2.0_f64 * t2040 * t96686 - 2.0_f64 * t28852 * t7042 - 4.0_f64 * t28855 * t7042 - 2.0_f64 * t31532 * t5494 + t128474 - t128475 - t128477 - t128482 - t128485 - t128492 - t128498 + t128502 - t128509 - t128511 - t128513 - t8329;
    t128514
}
