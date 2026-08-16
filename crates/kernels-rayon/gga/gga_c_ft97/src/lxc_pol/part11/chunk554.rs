//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 554/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk554(t1586: f64, t378: f64, t1588: f64, t379: f64, t446: f64, t1647: f64, t432: f64, t1564: f64, t1656: f64, t6: f64, t1602: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7824 = t378 * t1586;
    let t7825 = t379 * t1588;
    let t7826 = t7824 * t7825;
    let t7827 = t446 * t7826;
    let t7829 = t1647 * t432;
    let t7830 = t1564 * t7829;
    let t7831 = t446 * t7830;
    let t7833 = t1656 * t6;
    let t7837 = t1602 * t66;
    (t7824, t7825, t7826, t7827, t7829, t7830, t7831, t7833, t7837)
}
