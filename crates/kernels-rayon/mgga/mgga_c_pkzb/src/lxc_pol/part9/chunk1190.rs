//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1190/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1190(t1867: f64, t667: f64, t7375: f64, t7378: f64, t1281: f64, t204: f64, t2739: f64) -> (f64, f64, f64) {
    let t20707 = t1867 * t667;
    let t20708 = t7375 * t20707;
    let t20710 = t7378 * t20707;
    let t20716 = t204 * t1281 * t2739;
    (t20708, t20710, t20716)
}
