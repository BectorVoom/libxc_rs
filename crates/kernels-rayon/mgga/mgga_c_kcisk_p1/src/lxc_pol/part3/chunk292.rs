//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 292/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk292(t1163: f64, t1375: f64, t79: f64, t963: f64, t435: f64, t437: f64, t313: f64) -> (f64, f64, f64, f64) {
    let t1376 = t1375 * t1163;
    let t1379 = t963 * t79;
    let t1382 = 0.7925e-3_f64 * t435 * t1379 * t437;
    let t1383 = t79 * t313;
    (t1376, t1379, t1382, t1383)
}
