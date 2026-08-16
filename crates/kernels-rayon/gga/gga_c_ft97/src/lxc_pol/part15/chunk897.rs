//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 897/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk897(t1160: f64, t9802: f64, t9895: f64, t1087: f64, t3704: f64, t89: f64, t1611: f64, t806: f64, t1609: f64, t9523: f64, t1092: f64, t3051: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51990 = t9802 * t1160;
    let t52006 = t9895 * t1160;
    let t52212 = t89 * t3704 * t1087;
    let t52324 = t1611 * t806;
    let t52358 = t1609 * t9523;
    let t52453 = t3051 * t1092;
    (t51990, t52006, t52212, t52324, t52358, t52453)
}
