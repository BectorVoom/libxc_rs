//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1099/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1099(t310: f64, t6415: f64, t1915: f64, t3896: f64, t15115: f64, t557: f64, t1658: f64, t16986: f64, t6472: f64, t1814: f64, t441: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19704 = t310 * t6415;
    let t19706 = t3896 * t1915;
    let t19708 = t15115 * t557;
    let t19711 = t1658 * t1658;
    let t19716 = t16986 * t6472;
    let t19718 = t441 * t1814;
    (t19704, t19706, t19708, t19711, t19716, t19718)
}
