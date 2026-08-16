//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1220/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1220(t1030: f64, t1971: f64, t9267: f64, t9272: f64, t19522: f64, t33623: f64, t5541: f64, t11303: f64, t19588: f64, t1743: f64, t33373: f64, t5967: f64) -> (f64, f64, f64, f64, f64) {
    let t35105 = t1030 * t1971 * t9267 * t9272;
    let t35108 = t5541 * t33623 * t19522;
    let t35110 = t11303 * t19588;
    let t35112 = t1743 * t33373;
    let t35113 = t35112 * t5967;
    (t35105, t35108, t35110, t35112, t35113)
}
