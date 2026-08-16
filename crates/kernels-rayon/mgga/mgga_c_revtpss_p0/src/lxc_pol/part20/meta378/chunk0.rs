//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1370/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1370(t10871: f64, t40262: f64, t14917: f64, t2475: f64, t2661: f64, t2662: f64, t836: f64, t2749: f64, t40378: f64, t2430: f64, t853: f64, t837: f64) -> (f64, f64, f64, f64, f64) {
    let t40537 = t40262 * t10871;
    let t40549 = t2661 * t2662 * t2475 * t836 * t14917;
    let t40553 = t2661 * t2662 * t40378 * t2749;
    let t40555 = t853 * t2430;
    let t40558 = t2661 * t2662 * t40555 * t837;
    (t40537, t40549, t40553, t40555, t40558)
}
