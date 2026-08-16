//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1195/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1195(t19681: f64, t2535: f64, t2371: f64, t19575: f64, t592: f64, t2221: f64, t6328: f64, t2223: f64, t2225: f64, t17: f64, t2516: f64, t6320: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56104 = t19681 * t2535;
    let t56168 = t19681 * t2371;
    let t56185 = t592 * t19575;
    let t56390 = t2221 * t6328;
    let t56392 = t2223 * t6328;
    let t56394 = t2225 * t6328;
    let t56398 = t17 * t6320 * t2516;
    (t56104, t56168, t56185, t56390, t56392, t56394, t56398)
}
