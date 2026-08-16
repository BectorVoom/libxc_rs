//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 884/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk884(t301: f64, t761: f64, t9539: f64, t758: f64, t154: f64, t2048: f64, t3515: f64, t276: f64, t742: f64, t9161: f64, t779: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9541 = t301 * t9539 * t761;
    let t9542 = t758 * t9541;
    let t9546 = t154 * t2048 * t3515;
    let t9547 = t276 * t9546;
    let t9550 = t154 * t742 * t9161;
    let t9553 = t779 * t3515;
    let t9554 = t9553 * t655;
    (t9541, t9542, t9546, t9547, t9550, t9554)
}
