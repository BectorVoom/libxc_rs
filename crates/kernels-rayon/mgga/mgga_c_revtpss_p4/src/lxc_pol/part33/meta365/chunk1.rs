//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1398/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1398(t14616: f64, t757: f64, t1544: f64, t2475: f64, t124: f64, t1558: f64, t10779: f64, t2749: f64, t10777: f64, t125: f64, t4423: f64, t136: f64, t243: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14618 = 0.36622894612013090108e-3_f64 * t14616 * t757;
    let t14648 = t2475 * t1544;
    let t14671 = t124 * t1558;
    let t14673 = t10779 * t14671 * t2749;
    let t14675 = 0.10164000561857065645e-3_f64 * t10777 * t14673;
    let t14676 = t125 * t4423;
    let t14685 = t243 * t136;
    (t14618, t14648, t14671, t14673, t14675, t14676, t14685)
}
