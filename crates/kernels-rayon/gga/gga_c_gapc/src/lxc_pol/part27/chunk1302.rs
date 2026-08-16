//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1302/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1302(t11616: f64, t3212: f64, t10366: f64, t11613: f64, t3209: f64, t11682: f64, t23678: f64, t2415: f64, t2546: f64, t11612: f64, t2300: f64, t3723: f64) -> (f64, f64, f64, f64, f64) {
    let t36009 = t3212 * t11616;
    let t36011 = t10366 * t11613;
    let t36013 = t3209 * t11616;
    let t36017 = t11682 * t2415 * t2546 * t23678;
    let t36020 = t11612 * t3723 * t2300;
    (t36009, t36011, t36013, t36017, t36020)
}
