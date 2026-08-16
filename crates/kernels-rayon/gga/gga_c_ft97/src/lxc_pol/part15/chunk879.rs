//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 879/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk879(t41950: f64, t1636: f64, t2344: f64, t375: f64, t9567: f64, t241: f64, t41446: f64, t190: f64, t2371: f64, t251: f64, t36452: f64, t37991: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41951 = 140.0_f64 / 243.0_f64 * t41950;
    let t41955 = t1636 * t2344;
    let t41962 = t375 * t9567;
    let t41966 = t241 * t41446;
    let t42044 = 280.0_f64 / 243.0_f64 * t41950;
    let t42050 = 1.0_f64 / t251 / t37991 / t190 / t2371 / t36452 / 96.0_f64;
    (t41951, t41955, t41962, t41966, t42044, t42050)
}
