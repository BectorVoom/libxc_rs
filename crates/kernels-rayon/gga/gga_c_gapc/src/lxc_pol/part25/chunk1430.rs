//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1430/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1430(t33407: f64, t36570: f64, t36571: f64, t36572: f64, t36573: f64, t36574: f64, t36575: f64, t36577: f64, t36578: f64, t36579: f64, t36580: f64, t33464: f64, t33474: f64, t36596: f64, t36597: f64, t36599: f64, t36600: f64, t36601: f64, t36602: f64, t36604: f64, t36605: f64, t36606: f64) -> (f64, f64) {
    let t38728 = -t36570 + t36571 - t36572 - t36573 + t36574 - t36575 - 0.36231816839129402172e-6_f64 * t33407 + t36577 + t36578 + t36579 - t36580;
    let t38740 = -t36596 - t36597 - 0.18115908419564701086e-6_f64 * t33464 + t36599 - t36600 + t36601 + t36602 - 0.56912804804009946682e-7_f64 * t33474 - t36604 + t36605 + t36606;
    (t38728, t38740)
}
