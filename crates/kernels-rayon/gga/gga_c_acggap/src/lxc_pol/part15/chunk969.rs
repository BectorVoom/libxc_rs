//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 969/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk969(t13462: f64, t2065: f64, t2450: f64, t56: f64, t30321: f64, t1581: f64, t7614: f64, t2327: f64, t7780: f64, t30325: f64, t30318: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34278 = t2450 * t2065 * t56 * t13462;
    let t34283 = 0.42874018118069736972e-3_f64 * t30321;
    let t34284 = t7614 * t1581;
    let t34286 = t7780 * t2327;
    let t34288 = 0.18868855373762491241e-2_f64 * t30325;
    let t34293 = t30318 * t532;
    (t34278, t34283, t34284, t34286, t34288, t34293)
}
