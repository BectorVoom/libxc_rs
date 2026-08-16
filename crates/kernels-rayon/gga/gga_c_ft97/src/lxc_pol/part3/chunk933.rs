//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 933/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk933(t1882: f64, t5149: f64, t1091: f64, t3864: f64, t14175: f64, t1168: f64, t505: f64, t3699: f64, t14182: f64, t3690: f64, t14187: f64, t3859: f64) -> (f64, f64, f64, f64, f64) {
    let t18431 = t1882 * t5149;
    let t18433 = t1091 * t3864;
    let t18434 = t14175 * t18433;
    let t18437 = t1168 * t505;
    let t18438 = t3699 * t18437;
    let t18439 = t14182 * t18438;
    let t18442 = t3690 * t18437;
    let t18443 = t14187 * t18442;
    let t18446 = t1091 * t3859;
    (t18431, t18434, t18439, t18443, t18446)
}
