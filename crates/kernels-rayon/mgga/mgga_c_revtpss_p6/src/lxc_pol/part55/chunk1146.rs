//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1146/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1146(t121166: f64, t25304: f64, t8571: f64, t121035: f64, t32268: f64, t32733: f64, t531: f64, t2411: f64, t32486: f64, t198: f64, t206: f64, t8656: f64) -> (f64, f64, f64, f64, f64) {
    let t121363 = t25304 * t8571 * t121166;
    let t121365 = t32268 * t121035;
    let t121593 = t531 * t32733;
    let t121716 = t32486 * t2411;
    let t121751 = t198 * t206 * t8656;
    (t121363, t121365, t121593, t121716, t121751)
}
