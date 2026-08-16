//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 634/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk634(t100: f64, t8216: f64, t1882: f64, t1917: f64, t1878: f64, t1541: f64, t443: f64, t444: f64) -> (f64, f64, f64, f64) {
    let t8217 = t8216 * t100;
    let t8227 = t1882 * t1917;
    let t8229 = t1882 * t1878;
    let t8232 = t443 * t444 * t1541;
    (t8217, t8227, t8229, t8232)
}
