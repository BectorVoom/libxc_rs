//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 811/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk811(t1021: f64, t12499: f64, t181: f64, t197: f64, t3107: f64, t944: f64, t955: f64, t28: f64, t2883: f64, t14: f64, t2857: f64, t829: f64) -> (f64, f64, f64, f64, f64) {
    let t12500 = t1021 * t12499;
    let t12503 = t197 * t181;
    let t12505 = t944 * t955 * t3107;
    let t12512 = 1.0_f64 / t2883 / t28;
    let t12513 = t14 * t12512;
    let t12514 = t2857 * t829;
    (t12500, t12503, t12505, t12513, t12514)
}
