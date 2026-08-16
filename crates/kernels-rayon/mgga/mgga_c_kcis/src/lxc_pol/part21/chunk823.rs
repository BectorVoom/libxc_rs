//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 823/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk823(t3069: f64, t331: f64, t1027: f64, t3097: f64, t308: f64, t9758: f64, t1042: f64, t2943: f64, t3093: f64, t932: f64, t9725: f64, t2861: f64, t3184: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10192 = t331 * t3069;
    let t10194 = t1027 * t3097;
    let t10199 = t9758 * t308;
    let t10202 = t2943 * t1042;
    let t10208 = t932 * t3093;
    let t10218 = 0.12841111111111111111e-1_f64 * t9725;
    let t10243 = t2861 * t3184;
    (t10192, t10194, t10199, t10202, t10208, t10218, t10243)
}
