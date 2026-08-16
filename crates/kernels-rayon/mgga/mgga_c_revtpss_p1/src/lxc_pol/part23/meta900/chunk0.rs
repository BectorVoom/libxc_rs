//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2861/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2861(t77047: f64, t14330: f64, t18575: f64, t4186: f64, t18259: f64, t18306: f64, t23210: f64, t705: f64, t707: f64, t1522: f64, t61122: f64, t40205: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77048 = 0.5848223622634646207e0_f64 * t77047;
    let t77051 = 72.0_f64 * t14330 * t18575 * t4186;
    let t77053 = 36.0_f64 * t18259 * t18306;
    let t77054 = t705 * t23210;
    let t77056 = 4.0_f64 * t77054 * t707;
    let t77058 = 12.0_f64 * t61122 * t1522;
    let t77059 = 0.35089341735807877242e1_f64 * t40205;
    (t77048, t77051, t77053, t77056, t77058, t77059)
}
