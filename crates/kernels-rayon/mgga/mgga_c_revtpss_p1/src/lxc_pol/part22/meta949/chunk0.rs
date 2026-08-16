//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3189/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3189(t29048: f64, t3362: f64, t3655: f64, t5258: f64, t5262: f64, t12976: f64, t5362: f64, t12963: f64, t5327: f64, t12995: f64, t17308: f64, t17283: f64, t3678: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t59330 = t29048 * t3362;
    let t59336 = t5258 * t3655;
    let t59338 = t5262 * t3655;
    let t59349 = t12976 * t5362;
    let t59351 = t5327 * t12963;
    let t59353 = t17308 * t12995;
    let t59358 = t17283 * t3678;
    (t59330, t59336, t59338, t59349, t59351, t59353, t59358)
}
