//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1300/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1300(t67179: f64, t67185: f64, t46302: f64, t67209: f64, t16: f64, t39031: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75894 = 48.0_f64 * t67179;
    let t75895 = 96.0_f64 * t67185;
    let t75900 = 0.4155806185363551302e3_f64 * t46302;
    let t75901 = 0.73245789224026180216e-3_f64 * t67209;
    let t75910 = t16 + t39031;
    let t75911 = 24.0_f64 * t75910;
    (t75894, t75895, t75900, t75901, t75910, t75911)
}
