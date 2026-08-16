//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1721/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1721(t7315: f64, t7536: f64, t25089: f64, t7488: f64, t2107: f64, t25802: f64, t1310: f64, t7373: f64, t116: f64, t7356: f64) -> (f64, f64, f64, f64, f64) {
    let t26380 = t7536 * t7315;
    let t26383 = t7488 * t25089;
    let t26392 = t2107 * t25802;
    let t26396 = t1310 * t7373;
    let t26399 = t7356 * t116;
    (t26380, t26383, t26392, t26396, t26399)
}
