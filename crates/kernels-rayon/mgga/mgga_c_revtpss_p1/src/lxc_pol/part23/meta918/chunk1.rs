//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2960/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2960(t78446: f64, t78449: f64, t78451: f64, t78456: f64, t78458: f64, t78460: f64, t78463: f64, t78465: f64, t78469: f64, t78472: f64, t78474: f64, t24186: f64, t3336: f64) -> (f64, f64) {
    let t78475 = -t78446 + t78449 + t78451 - t78456 + t78458 + t78460 + t78463 - t78465 + t78469 - t78472 - t78474;
    let t78478 = t24186 * t3336;
    (t78475, t78478)
}
