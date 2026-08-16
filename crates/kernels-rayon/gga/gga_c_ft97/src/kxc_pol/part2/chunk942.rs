//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 942/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk942(t14621: f64, t3281: f64, t1091: f64, t2739: f64, t2665: f64, t446: f64, t3746: f64, t824: f64, t13296: f64, t835: f64, t1882: f64, t4038: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14622 = t3281 * t14621;
    let t14624 = t1091 * t2739;
    let t14625 = t2665 * t14624;
    let t14626 = t446 * t14625;
    let t14628 = t3746 * t824;
    let t14629 = t2665 * t14628;
    let t14630 = t3281 * t14629;
    let t14632 = t835 * t13296;
    let t14633 = t446 * t14632;
    let t14635 = t1882 * t4038;
    (t14622, t14624, t14626, t14628, t14630, t14633, t14635)
}
