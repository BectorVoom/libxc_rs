//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1111/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1111(t1878: f64, t209: f64, t81982: f64, t6604: f64, t9971: f64, t206: f64, t22723: f64, t268: f64, t23163: f64, t1879: f64, t80845: f64, t1906: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81984 = t1878 * t81982 * t209;
    let t82018 = t6604 * t9971;
    let t82031 = t22723 * t206 * t268;
    let t82038 = t22723 * t23163;
    let t82045 = t80845 * t1879;
    let t82046 = t82045 * t1906;
    (t81984, t82018, t82031, t82038, t82045, t82046)
}
