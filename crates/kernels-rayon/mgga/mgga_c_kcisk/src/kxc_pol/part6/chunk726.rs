//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 726/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk726(t12998: f64, t12974: f64, t1173: f64, t1337: f64, t459: f64, t1354: f64, t1422: f64, t306: f64, t3529: f64, t3530: f64, t425: f64, t3598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13091 = 0.36793333333333333333e0_f64 * t12998;
    let t13092 = 0.93932222222222222223e0_f64 * t12974;
    let t13110 = 0.55403703703703703703e-1_f64 * t12974;
    let t13129 = t1337 * t1173 * t459;
    let t13138 = t1422 * t1354;
    let t13148 = t3529 * t306 * t459;
    let t13153 = t3530 * t425;
    let t13185 = t3598 * t459;
    (t13091, t13092, t13110, t13129, t13138, t13148, t13153, t13185)
}
