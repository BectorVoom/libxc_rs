//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 563/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk563(t2: f64, t8326: f64, t369: f64, t631: f64, t637: f64, t7242: f64, t96: f64, t375: f64, t443: f64, t444: f64) -> (f64, f64, f64) {
    let t8327 = t8326 * t2;
    let t8345 = 1.0_f64 / t96 / t631 / t637 / t369 / t7242 / 4.0_f64;
    let t8392 = t443 * t444 * t375;
    (t8327, t8345, t8392)
}
