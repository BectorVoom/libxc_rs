//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 641/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk641(t1882: f64, t1897: f64, t1893: f64, t454: f64, t8232: f64, t1855: f64, t1913: f64, t8392: f64, t463: f64, t480: f64, t1637: f64, t482: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8477 = t1882 * t1897;
    let t8483 = t1882 * t1893;
    let t8485 = t8232 * t454;
    let t8487 = t1882 * t1855;
    let t8499 = t8392 * t1913;
    let t8506 = t463 * t480;
    let t8516 = t89 * t1637 * t482;
    (t8477, t8483, t8485, t8487, t8499, t8506, t8516)
}
