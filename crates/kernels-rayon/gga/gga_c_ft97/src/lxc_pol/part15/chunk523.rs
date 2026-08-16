//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 523/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk523(t291: f64, t5266: f64, t1208: f64, t3780: f64, t1701: f64) -> (f64, f64, f64, f64) {
    let t5267 = t5266 * t291;
    let t5272 = t3780 * t1208;
    let t5273 = t1701 * t5272;
    let t5284 = t1208 * t1208;
    (t5267, t5272, t5273, t5284)
}
