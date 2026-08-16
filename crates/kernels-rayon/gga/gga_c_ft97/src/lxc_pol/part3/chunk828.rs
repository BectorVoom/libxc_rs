//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 828/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk828(t139: f64, t16887: f64, t527: f64, t1008: f64, t132: f64, t1013: f64, t12367: f64, t1995: f64, t4699: f64, t4703: f64, t542: f64, t4698: f64, t549: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16888 = t139 * t16887;
    let t16889 = t527 * t16888;
    let t16891 = t1008 * t132;
    let t16894 = t12367 * t1013;
    let t16897 = t1995 * t4699;
    let t16902 = t542 * t4703;
    let t16907 = t549 * t4698;
    (t16889, t16891, t16894, t16897, t16902, t16907)
}
