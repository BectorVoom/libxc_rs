//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 729/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk729(t3103: f64, t452: f64, t499: f64, t110: f64, t11392: f64, t1882: f64, t3257: f64, t1786: f64, t11397: f64, t463: f64, t488: f64, t1911: f64, t2992: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11459 = t452 * t499 * t3103;
    let t11463 = t452 * t110 * t11392;
    let t11467 = 2.0_f64 / 9.0_f64 * t1882 * t3257;
    let t11468 = t1786 * t110;
    let t11469 = t11468 * t11397;
    let t11472 = t463 * t488;
    let t11473 = t2992 * t1911;
    (t11459, t11463, t11467, t11469, t11472, t11473)
}
