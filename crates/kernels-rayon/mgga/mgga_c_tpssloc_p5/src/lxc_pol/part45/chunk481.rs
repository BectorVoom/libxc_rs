//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 481/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk481(t1186: f64, t135: f64, t1174: f64, t1089: f64, t405: f64, t974: f64, t3242: f64, t461: f64, t2244: f64, t337: f64, t51: f64, t1887: f64) -> (f64, f64, f64, f64) {
    let t3435 = t135 * t1186;
    let t3436 = t1174 * t3435;
    let t3439 = 1.0_f64 / t405 / t1089;
    let t3440 = t974 * t3439;
    let t3441 = t461 * t3242;
    let t3442 = t3441 * t2244;
    let t3443 = t3440 * t3442;
    let t3446 = t51 * t337;
    let t3447 = t3446 * t1887;
    (t3436, t3439, t3443, t3447)
}
