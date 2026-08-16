//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 816/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk816(t157: f64, t9224: f64, t160: f64, t7763: f64, t7800: f64, t1570: f64, t586: f64, t1557: f64, t2: f64, t1985: f64, t2097: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12723 = t9224 * t157;
    let t12724 = t160 * t7763;
    let t12746 = t160 * t7800;
    let t12791 = t586 * t1570;
    let t12796 = t586 * t1557;
    let t12823 = t9224 * t2;
    let t12968 = t1985 * t157;
    let t12982 = t2097 * t597;
    (t12723, t12724, t12746, t12791, t12796, t12823, t12968, t12982)
}
