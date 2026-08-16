//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1378/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1378(t1424: f64, t2299: f64, t544: f64, t8070: f64, t10609: f64, t4781: f64, t10241: f64, t1323: f64, t20827: f64, t6717: f64, t10314: f64, t20441: f64, t6716: f64) -> (f64, f64, f64, f64, f64) {
    let t34454 = 0.79445533226334281486e-1_f64 * t544 * t2299 * t8070 * t1424;
    let t34457 = t4781 * t10609;
    let t34458 = 0.1533717038156829987e1_f64 * t34457;
    let t34459 = t10241 * t1323;
    let t34462 = 0.13803453343411469884e2_f64 * t20827 * t6717 * t34459;
    let t34465 = 0.18404604457881959845e2_f64 * t6716 * t20441 * t10314;
    (t34454, t34458, t34459, t34462, t34465)
}
