//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 894/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk894(t1068: f64, t3628: f64, t1075: f64, t1008: f64, t8907: f64, t1018: f64, t2999: f64, t89: f64, t3000: f64, t998: f64, t1045: f64, t9132: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48117 = t3628 * t1068;
    let t48442 = t3628 * t1075;
    let t48636 = t8907 * t1008;
    let t49266 = t89 * t2999 * t1018;
    let t49337 = t89 * t3000 * t998;
    let t49622 = t9132 * t1045;
    (t48117, t48442, t48636, t49266, t49337, t49622)
}
