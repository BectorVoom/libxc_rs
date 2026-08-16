//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 519/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk519(t1780: f64, t2: f64, t463: f64, t17: f64, t3050: f64, t9: f64, t103: f64, t1570: f64, t100: f64) -> (f64, f64, f64, f64, f64) {
    let t3127 = t1780 * t2;
    let t3134 = t463 * t2;
    let t3139 = t9 * t3050 * t17;
    let t3187 = t103 * t1570;
    let t3193 = t1780 * t100;
    (t3127, t3134, t3139, t3187, t3193)
}
