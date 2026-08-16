//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 873/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk873(t12719: f64, t409: f64, t3740: f64, t932: f64, t1159: f64, t848: f64, t1162: f64) -> (f64, f64, f64, f64) {
    let t12720 = t12719 * t409;
    let t12724 = t3740 * t932;
    let t12726 = t848 * t1159;
    let t12727 = t12726 * t1162;
    (t12720, t12724, t12726, t12727)
}
