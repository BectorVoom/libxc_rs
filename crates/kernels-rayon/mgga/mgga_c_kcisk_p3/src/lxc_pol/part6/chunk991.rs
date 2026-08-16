//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 991/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk991(t1248: f64, t1249: f64, t30238: f64, t30290: f64, t4065: f64, t30298: f64, t1242: f64, t30339: f64, t13666: f64, t13672: f64, t26198: f64, t30306: f64, t30353: f64, t30355: f64, t30357: f64, t30360: f64, t30363: f64, t30366: f64) -> (f64, f64, f64, f64, f64) {
    let t30369 = t1248 * t1249 * t30238;
    let t30372 = t1248 * t4065 * t30290;
    let t30375 = t1248 * t1249 * t30298;
    let t30377 = t1242 * t30339;
    let t30379 = -0.29896666666666666667e0_f64 * t30306 + 0.32862666666666666666e0_f64 * t26198 + 0.142419375e1_f64 * t30353 - t13666 - 0.28483875e1_f64 * t30355 + 0.46074375e0_f64 * t30357 + 0.98587999999999999998e0_f64 * t30360 - 0.16431333333333333333e0_f64 * t30363 - 0.73028148148148148146e-1_f64 * t30366 - 0.16431333333333333333e0_f64 * t30369 + 0.32862666666666666666e0_f64 * t30372 - 0.98587999999999999998e0_f64 * t30375 + 0.3071625e0_f64 * t30377 - t13672;
    (t30369, t30372, t30375, t30377, t30379)
}
