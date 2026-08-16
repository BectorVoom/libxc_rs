//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1037/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1037(t32374: f64, t670: f64, t572: f64, t2089: f64, t7002: f64, t651: f64, t8686: f64, t1936: f64, t648: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32375 = t32374 * t670;
    let t32376 = t572 * t32375;
    let t32377 = 6.0_f64 * t32376;
    let t32385 = t2089 * t7002;
    let t32386 = t651 * t32385;
    let t32387 = t8686 * t670;
    let t32388 = t651 * t32387;
    let t32392 = t648 * t1936;
    (t32375, t32377, t32385, t32386, t32387, t32388, t32392)
}
