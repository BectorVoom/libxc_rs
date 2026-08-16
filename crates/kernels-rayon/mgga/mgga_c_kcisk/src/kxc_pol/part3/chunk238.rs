//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 238/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk238(t1050: f64, t210: f64, t1033: f64, t1037: f64, t1040: f64, t1043: f64, t1046: f64) -> (f64, f64) {
    let t1051 = t210 * t1050;
    let t1053 = t1033 / 8.0_f64 - t1037 / 8.0_f64 - t1040 / 4.0_f64 - t1043 / 64.0_f64 + t1046 / 64.0_f64 + t1051 / 16.0_f64;
    (t1051, t1053)
}
