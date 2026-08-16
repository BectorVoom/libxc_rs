//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 884/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk884(t25253: f64, t25275: f64, t25283: f64, t122: f64, t2061: f64, t72: f64, t25412: f64, t25411: f64, t2466: f64, t25387: f64, t2062: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26462 = 0.30488190661738479625e-3_f64 * t25253;
    let t26468 = 35.0_f64 / 216.0_f64 * t25275;
    let t26471 = 0.10164000561857065645e-4_f64 * t25283;
    let t26481 = t2061 * t72 * t122;
    let t26482 = t26481 * t25412;
    let t26483 = t25411 * t26482;
    let t26485 = t26481 * t2466;
    let t26486 = t25387 * t26485;
    let t26496 = t2062 * t867;
    (t26462, t26468, t26471, t26482, t26483, t26485, t26486, t26496)
}
