//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 606/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk606(t1628: f64, t3585: f64, t3576: f64, t3556: f64, t524: f64, t3560: f64, t11218: f64, t600: f64, t568: f64, t11254: f64, t447: f64, t1445: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11501 = t1628 * t3585;
    let t11504 = t1628 * t3576;
    let t11513 = t524 * t3556;
    let t11516 = t524 * t3560;
    let t11523 = t600 * t11218;
    let t11524 = t568 * t11523;
    let t11527 = t11254 * t447;
    let t11528 = t1445 * t11527;
    (t11501, t11504, t11513, t11516, t11524, t11528)
}
