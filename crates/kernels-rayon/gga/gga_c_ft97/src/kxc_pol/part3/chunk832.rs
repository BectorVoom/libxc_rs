//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 832/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk832(t16955: f64, t9127: f64, t2210: f64, t13153: f64, t3446: f64, t160: f64, t4714: f64, t379: f64, t2221: f64, t1882: f64, t4726: f64, t4805: f64, t558: f64) -> (f64, f64, f64, f64, f64) {
    let t16956 = t9127 * t16955;
    let t16957 = t2210 * t16956;
    let t16960 = t13153 * t3446;
    let t16963 = t160 * t4714;
    let t16964 = t16963 * t379;
    let t16965 = t2221 * t16964;
    let t16969 = t1882 * t4726;
    let t16971 = t4805 * t558;
    (t16957, t16960, t16965, t16969, t16971)
}
