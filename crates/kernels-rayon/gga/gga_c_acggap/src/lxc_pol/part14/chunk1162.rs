//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1162/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1162(t13287: f64, t2302: f64, t31057: f64, t8406: f64, t7447: f64, t9701: f64, t7440: f64, t9705: f64, t1314: f64, t507: f64, t8806: f64, t34406: f64, t6324: f64) -> (f64, f64, f64, f64, f64) {
    let t40043 = t31057 * t13287 * t2302 * t8406;
    let t40045 = t7447 * t9701;
    let t40047 = t7440 * t9705;
    let t40050 = t8806 * t507 * t1314;
    let t40054 = t34406 * t6324;
    (t40043, t40045, t40047, t40050, t40054)
}
