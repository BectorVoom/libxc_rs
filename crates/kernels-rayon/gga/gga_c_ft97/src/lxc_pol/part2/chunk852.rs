//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 852/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk852(t13301: f64, t3917: f64, t1775: f64, t3918: f64, t3911: f64, t1934: f64, t3690: f64) -> (f64, f64, f64, f64) {
    let t13302 = t3917 * t13301;
    let t13306 = 4.0_f64 / 9.0_f64 * t1775 * t3918;
    let t13308 = 4.0_f64 / 27.0_f64 * t1775 * t3911;
    let t13309 = t3690 * t1934;
    (t13302, t13306, t13308, t13309)
}
