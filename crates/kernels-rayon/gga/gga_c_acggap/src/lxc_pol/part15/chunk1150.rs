//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1150/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1150(t15386: f64, t31195: f64, t39891: f64, t13287: f64, t2297: f64, t5616: f64, t30817: f64, t9649: f64, t2030: f64, t507: f64, t8816: f64, t1488: f64, t2060: f64, t2317: f64) -> (f64, f64, f64, f64, f64) {
    let t39893 = t31195 * t15386 * t39891;
    let t39897 = t31195 * t13287 * t2297 * t5616;
    let t39899 = t30817 * t9649;
    let t39907 = t2030 * t507 * t8816;
    let t39910 = t2060 * t1488 * t2317;
    (t39893, t39897, t39899, t39907, t39910)
}
