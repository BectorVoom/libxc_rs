//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1235/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1235(t32387: f64, t4248: f64, t116: f64, t34187: f64, t13426: f64, t8641: f64, t18227: f64, t32401: f64, t34258: f64, t7374: f64, t648: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t128289 = t4248 * t32387;
    let t128291 = t34187 * t116;
    let t128293 = t13426 * t8641;
    let t128294 = t18227 * t8641;
    let t128295 = t4248 * t32401;
    let t128301 = t34258 * t7374;
    let t128302 = t648 * t7741;
    (t128289, t128291, t128293, t128294, t128295, t128301, t128302)
}
