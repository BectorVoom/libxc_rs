//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 659/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk659(t182: f64, t5299: f64, t1647: f64, t879: f64, t381: f64, t456: f64, t5080: f64, t1651: f64, t955: f64, t322: f64, t545: f64, t407: f64) -> (f64, f64, f64, f64, f64) {
    let t5300 = t182 * t5299;
    let t5304 = t1647 * t879;
    let t5305 = t381 * t5304;
    let t5307 = t456 * t5080;
    let t5310 = t1651 * t955;
    let t5315 = t545 * t322;
    let t5316 = t5315 * t407;
    (t5300, t5305, t5307, t5310, t5316)
}
