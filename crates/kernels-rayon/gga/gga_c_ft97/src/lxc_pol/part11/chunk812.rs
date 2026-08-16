//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 812/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk812(t463: f64, t488: f64, t100: f64, t370: f64, t110: f64, t8326: f64, t1780: f64, t480: f64, t2: f64, t8275: f64, t1555: f64, t26: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11472 = t463 * t488;
    let t11490 = t370 * t100;
    let t11552 = t8326 * t110;
    let t11556 = t1780 * t488;
    let t11587 = t1780 * t480;
    let t11690 = t8275 * t2;
    let t11755 = t26 * t1555;
    (t11472, t11490, t11552, t11556, t11587, t11690, t11755)
}
