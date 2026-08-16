//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 826/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk826(t1854: f64, t7351: f64, t1181: f64, t7564: f64, t1750: f64, t7561: f64, t1713: f64, t579: f64, t336: f64, t7400: f64, t1782: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9607 = t7351 * t1854;
    let t9608 = t1181 * t9607;
    let t9609 = t7564 * t9608;
    let t9611 = t7561 * t1750;
    let t9613 = t579 * t1713;
    let t9614 = t336 * t9613;
    let t9615 = t7400 * t9614;
    let t9617 = t604 * t1782;
    (t9607, t9608, t9609, t9611, t9614, t9615, t9617)
}
