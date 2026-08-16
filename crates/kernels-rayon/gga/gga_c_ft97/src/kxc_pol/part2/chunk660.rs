//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 660/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk660(t173: f64, t703: f64, t1526: f64, t2322: f64, t2326: f64, t342: f64, t630: f64, t2347: f64, t240: f64, t2349: f64, t2360: f64, t1934: f64, t2321: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9483 = t173 * t703;
    let t9485 = t1526 * t9483 * t2322;
    let t9488 = t342 * t630 * t2326;
    let t9490 = t240 * t2347;
    let t9491 = t9490 * t2349;
    let t9498 = t240 * t2360;
    let t9499 = t9498 * t2349;
    let t9503 = t2321 * t1934;
    (t9483, t9485, t9488, t9491, t9499, t9503)
}
