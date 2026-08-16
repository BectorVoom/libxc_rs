//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 859/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk859(t68: f64, t9971: f64, t226: f64, t1519: f64, t2627: f64, t1543: f64, t2841: f64, t1540: f64, t2394: f64) -> (f64, f64, f64, f64) {
    let t13396 = t68 * t9971;
    let t13397 = t226 * t13396;
    let t13416 = t2627 * t1519;
    let t13520 = t1543 * t2841;
    let t13598 = t2394 * t1540;
    (t13397, t13416, t13520, t13598)
}
