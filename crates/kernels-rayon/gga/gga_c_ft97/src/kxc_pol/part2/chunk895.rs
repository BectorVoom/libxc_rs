//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 895/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk895(t13863: f64, t3892: f64, t2606: f64, t265: f64, t668: f64, t724: f64, t1144: f64, t8232: f64, t1882: f64, t3991: f64, t3887: f64, t9787: f64) -> (f64, f64, f64, f64, f64) {
    let t13864 = t3892 * t13863;
    let t13865 = t2606 * t13864;
    let t13869 = t724 * t265 * t668;
    let t13872 = t8232 * t1144;
    let t13875 = 2.0_f64 / 9.0_f64 * t1882 * t3991;
    let t13876 = t9787 * t3887;
    (t13865, t13869, t13872, t13875, t13876)
}
