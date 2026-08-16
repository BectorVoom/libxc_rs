//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1142/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1142(t10309: f64, t121646: f64, t119457: f64, t1925: f64, t32589: f64, t8442: f64, t45963: f64, t8619: f64, t32597: f64, t32602: f64, t2411: f64, t32486: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121647 = t10309 * t121646;
    let t121656 = t119457 * t1925;
    let t121660 = t10309 * t32589;
    let t121661 = t8442 * t1925;
    let t121665 = t45963 * t8619;
    let t121689 = t32597 * t32602;
    let t121716 = t32486 * t2411;
    (t121647, t121656, t121660, t121661, t121665, t121689, t121716)
}
