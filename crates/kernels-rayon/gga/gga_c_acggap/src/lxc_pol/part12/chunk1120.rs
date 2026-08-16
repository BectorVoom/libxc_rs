//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1120/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1120(t2095: f64, t33901: f64, t33884: f64, t1998: f64, t4503: f64, t5124: f64, t7647: f64, t7310: f64, t8878: f64, t1446: f64, t7614: f64, t2001: f64, t4542: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35913 = t2095 * t33901;
    let t35915 = t2095 * t33884;
    let t35917 = t1998 * t4503;
    let t35919 = t7647 * t5124;
    let t35924 = t7310 * t8878;
    let t35926 = t7614 * t1446;
    let t35928 = t2001 * t4542;
    (t35913, t35915, t35917, t35919, t35924, t35926, t35928)
}
