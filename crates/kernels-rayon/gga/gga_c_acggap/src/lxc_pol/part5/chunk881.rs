//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 881/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk881(t1106: f64, t1181: f64, t12936: f64, t991: f64, t1090: f64, t3361: f64, t1530: f64, t3402: f64, t922: f64, t944: f64, t1172: f64, t12935: f64) -> (f64, f64, f64, f64, f64) {
    let t12939 = t12936 * t1181 * t991 * t1106;
    let t12943 = t3361 * t1181 * t991 * t1090;
    let t12945 = t1530 * t3402;
    let t12946 = t944 * t922;
    let t12991 = t12935 * t1172;
    (t12939, t12943, t12945, t12946, t12991)
}
