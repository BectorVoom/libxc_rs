//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 477/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk477(t2749: f64, t875: f64, t296: f64, t304: f64, t305: f64, t856: f64, t91: f64, t1771: f64, t303: f64, t1775: f64, t849: f64, t458: f64, t854: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2750 = t2749 * t875;
    let t2751 = t296 * t2750;
    let t2755 = 1.0_f64 / t305 / t304;
    let t2756 = t856 * t856;
    let t2758 = t91 * t2755 * t2756;
    let t2761 = 4.0_f64 / 9.0_f64 * t1771 * t303;
    let t2762 = t1775 * t849;
    let t2764 = t458 * t854;
    (t2750, t2751, t2755, t2756, t2758, t2761, t2762, t2764)
}
