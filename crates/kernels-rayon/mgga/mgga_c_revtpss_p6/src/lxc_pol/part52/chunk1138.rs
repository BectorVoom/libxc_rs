//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1138/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1138(t121210: f64, t25304: f64, t8705: f64, t596: f64, t8571: f64, t32186: f64, t786: f64, t119833: f64, t121245: f64, t121248: f64, t121116: f64, t32208: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121275 = t25304 * t8705 * t121210;
    let t121305 = t8571 * t596;
    let t121307 = t786 * t121305 * t32186;
    let t121326 = t119833 * t121245;
    let t121327 = t121326 * t121248;
    let t121336 = t121116 * t32208;
    (t121275, t121305, t121307, t121326, t121327, t121336)
}
