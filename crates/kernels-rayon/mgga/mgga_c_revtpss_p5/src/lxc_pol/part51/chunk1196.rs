//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1196/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1196(t105823: f64, t572: f64, t7002: f64, t7331: f64, t7944: f64, t2040: f64, t28268: f64, t4292: f64, t8453: f64, t28265: f64, t28280: f64, t5795: f64, t8614: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t127480 = 12.0_f64 * t572 * t105823 * t7002;
    let t127481 = t7944 * t7331;
    let t127483 = t2040 * t28268;
    let t127489 = 6.0_f64 * t572 * t4292 * t8453;
    let t127490 = t2040 * t28265;
    let t127492 = t2040 * t28280;
    let t127495 = 3.0_f64 * t5795 * t8614;
    (t127480, t127481, t127483, t127489, t127490, t127492, t127495)
}
