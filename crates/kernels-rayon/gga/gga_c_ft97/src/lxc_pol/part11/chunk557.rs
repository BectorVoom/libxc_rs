//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 557/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk557(t12: f64, t52: f64, t1703: f64, t1593: f64, t1609: f64, t1595: f64, t1597: f64, t63: f64, t1620: f64, t5544: f64, t39: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7853 = t52 * t12;
    let t7854 = t7853 * t1703;
    let t7857 = t1609 * t1593;
    let t7858 = t1595 * t1597;
    let t7859 = t7858 * t63;
    let t7860 = t7857 * t7859;
    let t7861 = t5544 * t1620;
    let t7866 = t409 * t39;
    (t7853, t7854, t7857, t7858, t7859, t7860, t7861, t7866)
}
