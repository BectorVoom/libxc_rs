//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 990/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk990(t4121: f64, t491: f64, t1457: f64, t509: f64, t86: f64, t9526: f64, t1499: f64, t1491: f64, t1495: f64, t4161: f64, t1360: f64, t3960: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12240 = t4121 * sigma2;
    let t12241 = t12240 * t491;
    let t12265 = t1457 * t4121;
    let t12266 = t12265 * sigma2;
    let t12274 = t86 * t9526 * t509;
    let t12275 = t12274 * t1499;
    let t12279 = t12274 * t1491;
    let t12281 = t4161 * t1495;
    let t12286 = t1360 * t3960;
    (t12240, t12241, t12265, t12266, t12274, t12275, t12279, t12281, t12286)
}
