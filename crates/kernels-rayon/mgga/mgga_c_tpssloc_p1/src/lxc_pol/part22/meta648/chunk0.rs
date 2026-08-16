//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2188/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2188(t19573: f64, t588: f64, t592: f64, t118: f64, t2375: f64, t6320: f64, t12300: f64, t6422: f64, t12365: f64, t1358: f64, t19836: f64, t12250: f64, t6387: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57227 = t588 * t19573;
    let t57229 = t592 * t19573;
    let t57235 = t6320 * t118 * t2375;
    let t57308 = t12300 * t6422;
    let t57310 = t12365 * t6422;
    let t57324 = t19836 * t1358;
    let t57342 = t6387 * t12250;
    (t57227, t57229, t57235, t57308, t57310, t57324, t57342)
}
