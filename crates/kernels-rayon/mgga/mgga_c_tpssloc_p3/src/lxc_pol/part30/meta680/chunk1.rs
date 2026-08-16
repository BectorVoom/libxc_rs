//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2135/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2135(t28135: f64, t6914: f64, t1992: f64, t550: f64, t57607: f64, t6976: f64, t28168: f64, t57704: f64, t562: f64, t6347: f64, t1307: f64, t26331: f64, t26446: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96937 = t6914 * t28135;
    let t96941 = t1992 * t6976 * t57607 * t550;
    let t96945 = t6914 * t28168;
    let t96949 = t1992 * t6976 * t57704 * t550;
    let t96951 = t562 * t6347;
    let t96954 = t26331 * t26446 * t96951 * t1307;
    (t96937, t96941, t96945, t96949, t96951, t96954)
}
