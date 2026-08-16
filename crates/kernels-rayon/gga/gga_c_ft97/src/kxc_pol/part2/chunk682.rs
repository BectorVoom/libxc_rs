//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 682/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk682(t295: f64, t9567: f64, t2783: f64, t458: f64, t8282: f64, t849: f64, t1775: f64, t2778: f64, t2767: f64, t303: f64, t3051: f64, t1771: f64, t854: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10580 = t9567 * t295;
    let t10584 = t458 * t2783;
    let t10586 = t8282 * t849;
    let t10589 = t1775 * t2778;
    let t10591 = t1775 * t2767;
    let t10594 = 28.0_f64 / 27.0_f64 * t3051 * t303;
    let t10595 = t1771 * t854;
    (t10580, t10584, t10586, t10589, t10591, t10594, t10595)
}
