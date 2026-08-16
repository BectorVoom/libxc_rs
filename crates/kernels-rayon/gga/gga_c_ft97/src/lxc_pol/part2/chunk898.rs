//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 898/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk898(t3821: f64, t729: f64, t773: f64, t13672: f64, t265: f64, t1131: f64, t2619: f64, t2526: f64, t762: f64, t1160: f64, t2567: f64, t2569: f64) -> (f64, f64, f64, f64, f64) {
    let t13911 = t729 * t773 * t3821;
    let t13915 = t729 * t265 * t13672;
    let t13919 = t729 * t2619 * t1131;
    let t13922 = t1131 * t2526;
    let t13924 = t729 * t762 * t13922;
    let t13927 = t1160 * t2567;
    let t13928 = t13927 * t2569;
    (t13911, t13915, t13919, t13924, t13928)
}
