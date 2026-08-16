//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2586/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2586(t1261: f64, t12879: f64, t247: f64, t5056: f64, t225: f64, t56587: f64, t480: f64, t1214: f64, t3604: f64, t29048: f64, t3362: f64, t3655: f64, t5258: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t59233 = t1261 * t247 * t12879 * t5056;
    let t59241 = t56587 * t225;
    let t59242 = t59241 * t480;
    let t59279 = t3604 * t1214;
    let t59330 = t29048 * t3362;
    let t59336 = t5258 * t3655;
    (t59233, t59241, t59242, t59279, t59330, t59336)
}
