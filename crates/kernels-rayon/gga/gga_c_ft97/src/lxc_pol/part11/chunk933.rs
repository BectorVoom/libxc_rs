//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 933/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk933(t86: f64, t112: f64, t113: f64, t1927: f64, t1934: f64, t38381: f64, t39358: f64, t39370: f64, t5: f64, t502: f64, t505: f64, t8598: f64, t8608: f64) -> f64 {
    let t87 = 10000000.0_f64 <= t86;
    let t39375 = piecewise3(t87, 0.0_f64, t5 * (t38381 + t39358) * t113 / 4.0_f64 + t5 * t8598 * t505 + 3.0_f64 / 2.0_f64 * t5 * t1927 * t1934 + t5 * t502 * t8608 + t5 * t112 * t39370 / 4.0_f64);
    t39375
}
