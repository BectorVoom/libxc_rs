//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 990/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk990(t1165: f64, t12991: f64, t3655: f64, t4267: f64, t13087: f64, t4277: f64, t4528: f64, t997: f64, t1008: f64, t4542: f64, t1434: f64, t3244: f64) -> (f64, f64, f64, f64, f64) {
    let t16421 = t12991 * t1165 * t4267 * t3655;
    let t16423 = t13087 * t4277;
    let t16425 = t997 * t4528;
    let t16427 = t1008 * t4542;
    let t16438 = t3244 * t1434;
    (t16421, t16423, t16425, t16427, t16438)
}
