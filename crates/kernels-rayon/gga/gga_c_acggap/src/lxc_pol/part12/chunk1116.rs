//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1116/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1116(t7346: f64, t7347: f64, t8480: f64, t31350: f64, t4971: f64, t7447: f64, t8823: f64, t7440: f64, t8826: f64, t1488: f64, t2030: f64, t2031: f64) -> (f64, f64, f64, f64, f64) {
    let t35844 = t7346 * t8480 * t7347;
    let t35846 = t31350 * t4971;
    let t35848 = t7447 * t8823;
    let t35850 = t7440 * t8826;
    let t35853 = t2030 * t1488 * t2031;
    (t35844, t35846, t35848, t35850, t35853)
}
