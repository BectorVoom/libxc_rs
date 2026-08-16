//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 569/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk569(t435: f64, t919: f64, t3243: f64, t1936: f64, t2268: f64, t831: f64, t1062: f64, t268: f64, t2951: f64, t2208: f64, t2212: f64, t829: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3244 = t435 * t919;
    let t3245 = t3243 * t3244;
    let t3247 = t1936 * t919;
    let t3248 = t3243 * t3247;
    let t3250 = t2268 * t831;
    let t3251 = t1062 * t3250;
    let t3253 = t2951 * t268;
    let t3254 = t3253 * t2208;
    let t3255 = t829 * t2212;
    (t3244, t3245, t3247, t3248, t3250, t3251, t3253, t3254, t3255)
}
