//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 408/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk408(t2999: f64, t665: f64, t1132: f64, t375: f64, t89: f64, t1131: f64, t2371: f64, t223: f64, t226: f64) -> (f64, f64, f64, f64) {
    let t3704 = t2999 * t665;
    let t3710 = t89 * t375 * t1132;
    let t3717 = t2371 * t1131;
    let t3724 = t223 * t226;
    (t3704, t3710, t3717, t3724)
}
