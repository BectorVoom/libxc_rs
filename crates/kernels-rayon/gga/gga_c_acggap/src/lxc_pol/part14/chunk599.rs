//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 599/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk599(t145: f64, t1772: f64, t301: f64, t336: f64, t1795: f64, t429: f64, t1049: f64, t1765: f64, t1713: f64, t3132: f64, t345: f64, t1298: f64, t495: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5630 = t1772 * t145;
    let t5632 = t336 * t5630 * t301;
    let t5636 = t336 * t429 * t1795;
    let t5639 = t1049 * t1765;
    let t5641 = t1713 * t301;
    let t5642 = t3132 * t5641;
    let t5643 = t345 * t5642;
    let t5645 = t495 * t1298;
    (t5630, t5632, t5636, t5639, t5641, t5643, t5645)
}
