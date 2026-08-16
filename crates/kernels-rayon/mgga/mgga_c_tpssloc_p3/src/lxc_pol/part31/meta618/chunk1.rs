//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1868/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1868(t19885: f64, t90915: f64, t91004: f64, t28135: f64, t6914: f64, t1992: f64, t550: f64, t57607: f64, t6976: f64, t28168: f64, t57704: f64, t562: f64, t6347: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96935 = t91004 * t90915 * t19885;
    let t96937 = t6914 * t28135;
    let t96941 = t1992 * t6976 * t57607 * t550;
    let t96945 = t6914 * t28168;
    let t96949 = t1992 * t6976 * t57704 * t550;
    let t96951 = t562 * t6347;
    (t96935, t96937, t96941, t96945, t96949, t96951)
}
