//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 562/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk562(t2594: f64, t3691: f64, t446: f64, t1091: f64, t713: f64, t2354: f64, t2360: f64, t992: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3692 = t2594 * t3691;
    let t3693 = t446 * t3692;
    let t3695 = t1091 * t713;
    let t3696 = t2354 * t3695;
    let t3697 = t446 * t3696;
    let t3699 = t2360 * t992;
    (t3692, t3693, t3695, t3696, t3697, t3699)
}
