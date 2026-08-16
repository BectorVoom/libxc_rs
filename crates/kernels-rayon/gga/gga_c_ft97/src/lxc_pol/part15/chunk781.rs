//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 781/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk781(t21181: f64, t9717: f64, t89: f64, t9716: f64, t2348: f64, t666: f64, t21204: f64, t724: f64, t446: f64, t1131: f64, t4965: f64, t9744: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21431 = t9717 * t21181;
    let t21433 = t89 * t9716 * t21431;
    let t21435 = t2348 * t21181;
    let t21437 = t89 * t666 * t21435;
    let t21439 = t724 * t21204;
    let t21440 = t446 * t21439;
    let t21442 = t4965 * t1131;
    let t21443 = t9744 * t21442;
    (t21431, t21433, t21435, t21437, t21439, t21440, t21442, t21443)
}
