//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 926/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk926(t14220: f64, t3393: f64, t1530: f64, t3430: f64, t1037: f64, t1165: f64, t3451: f64, t930: f64, t879: f64, t944: f64, t3253: f64, t3456: f64) -> (f64, f64, f64, f64, f64) {
    let t14221 = t14220 * t3393;
    let t14223 = t1530 * t3430;
    let t14228 = t3451 * t1165 * t1037 * t930;
    let t14230 = t944 * t879;
    let t14233 = t3456 * t1165 * t3253 * t14230;
    (t14221, t14223, t14228, t14230, t14233)
}
