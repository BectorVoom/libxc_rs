//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1114/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1114(t33953: f64, t5127: f64, t13287: f64, t31057: f64, t4210: f64, t13364: f64, t31443: f64, t3169: f64, t2288: f64, t3176: f64, t17912: f64, t13299: f64, t31115: f64, t33938: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35284 = t33953 * t5127;
    let t35286 = t31057 * t13287 * t35284;
    let t35287 = 0.42874018118069736972e-3_f64 * t35286;
    let t35288 = t33953 * t4210;
    let t35290 = t31057 * t13364 * t35288;
    let t35291 = 0.21437009059034868486e-3_f64 * t35290;
    let t35294 = t31443 * t13287 * t33953 * t3169;
    let t35296 = t2288 * t3176;
    let t35298 = t31443 * t17912 * t35296;
    let t35301 = t31115 * t13299 * t33938;
    (t35284, t35287, t35288, t35291, t35294, t35296, t35298, t35301)
}
