//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1140/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1140(t121166: f64, t25304: f64, t8571: f64, t121035: f64, t32268: f64, t1455: f64, t8734: f64, t32733: f64, t531: f64, t32151: f64, t32597: f64, t10301: f64, t32589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121363 = t25304 * t8571 * t121166;
    let t121365 = t32268 * t121035;
    let t121531 = t1455 * t8734;
    let t121593 = t531 * t32733;
    let t121617 = t32597 * t32151;
    let t121625 = t10301 * t32589;
    (t121363, t121365, t121531, t121593, t121617, t121625)
}
