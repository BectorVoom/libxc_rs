//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 656/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk656(t159: f64, t1907: f64, t322: f64, t381: f64, t524: f64, t545: f64, t1539: f64, t1160: f64, t180: f64, t1814: f64, t3457: f64, t3073: f64) -> (f64, f64, f64, f64, f64) {
    let t6454 = t159 * t1907;
    let t6455 = t6454 * t322;
    let t6456 = t381 * t6455;
    let t6461 = t545 * t524;
    let t6462 = t6461 * t1539;
    let t6463 = t1160 * t6462;
    let t6465 = t180 * t1814;
    let t6466 = t6465 * t3457;
    let t6467 = t3073 * t6466;
    (t6456, t6461, t6463, t6465, t6467)
}
