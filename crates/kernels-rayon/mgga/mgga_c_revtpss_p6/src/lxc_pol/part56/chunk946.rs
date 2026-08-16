//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 946/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk946(t670: f64, t8453: f64, t572: f64, t7002: f64, t7330: f64, t1459: f64, t8614: f64, t116: f64, t8460: f64, t1936: f64, t648: f64, t94: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32366 = t670 * t8453;
    let t32368 = 6.0_f64 * t572 * t32366;
    let t32369 = t7330 * t7002;
    let t32371 = 12.0_f64 * t572 * t32369;
    let t32372 = t1459 * t8614;
    let t32373 = 3.0_f64 * t32372;
    let t32374 = t116 * t8460;
    let t32375 = t32374 * t670;
    let t32376 = t572 * t32375;
    let t32377 = 6.0_f64 * t32376;
    let t32392 = t648 * t1936;
    let t32394 = t94 * t7002;
    (t32366, t32368, t32369, t32371, t32373, t32374, t32375, t32377, t32392, t32394)
}
