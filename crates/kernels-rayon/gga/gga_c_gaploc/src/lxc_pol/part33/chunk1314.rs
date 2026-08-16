//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1314/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1314(t34441: f64, t10600: f64, t1397: f64, t1424: f64, t2875: f64, t544: f64, t6540: f64, t2299: f64, t8070: f64, t10609: f64, t4781: f64, t10241: f64, t1323: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34442 = 0.23005755572352449806e1_f64 * t34441;
    let t34445 = 0.79445533226334281486e-1_f64 * t1397 * t10600 * t1424;
    let t34449 = 0.79445533226334281486e-1_f64 * t544 * t6540 * t2875 * t1424;
    let t34454 = 0.79445533226334281486e-1_f64 * t544 * t2299 * t8070 * t1424;
    let t34457 = t4781 * t10609;
    let t34458 = 0.1533717038156829987e1_f64 * t34457;
    let t34459 = t10241 * t1323;
    (t34442, t34445, t34449, t34454, t34458, t34459)
}
