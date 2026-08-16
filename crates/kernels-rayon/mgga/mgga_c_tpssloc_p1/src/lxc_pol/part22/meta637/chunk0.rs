//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2175/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2175(t111: f64, t19449: f64, t19681: f64, t2528: f64, t172: f64, t19572: f64, t763: f64, t2535: f64, t2371: f64, t19575: f64, t592: f64, t1390: f64, t20063: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t55943 = t19449 * t111;
    let t56099 = t19681 * t2528;
    let t56102 = t19572 * t172 * t763;
    let t56104 = t19681 * t2535;
    let t56168 = t19681 * t2371;
    let t56185 = t592 * t19575;
    let t56358 = t20063 * t1390;
    (t55943, t56099, t56102, t56104, t56168, t56185, t56358)
}
