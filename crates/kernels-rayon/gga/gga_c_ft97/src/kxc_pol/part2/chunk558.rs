//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 558/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk558(t3653: f64, t637: f64, t639: f64, t2251: f64, t2254: f64, t2256: f64, t2265: f64, t3611: f64, t3614: f64, t3618: f64, t3622: f64, t3628: f64, t3630: f64, t3633: f64, t3637: f64, t3642: f64, t631: f64) -> (f64, f64) {
    let t3655 = t637 * t639 * t3653;
    let t3658 = -t2251 - t2254 / 9.0_f64 - t2256 / 3.0_f64 - t3611 / 9.0_f64 + t2265 * t3614 / 18.0_f64 - t2265 * t3618 / 3.0_f64 - t2265 * t3622 / 3.0_f64 + t3628 * t3630 / 3.0_f64 - t3633 / 3.0_f64 - t2265 * t3637 / 3.0_f64 - 3.0_f64 / 2.0_f64 * t631 * t3642 + t631 * t3655 / 2.0_f64;
    (t3655, t3658)
}
