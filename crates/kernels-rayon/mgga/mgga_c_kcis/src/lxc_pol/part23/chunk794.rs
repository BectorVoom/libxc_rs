//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 794/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk794(t12274: f64, t1499: f64, t3724: f64, t3728: f64, t1491: f64, t1495: f64, t4161: f64, t1360: f64, t3960: f64, t1460: f64, t3245: f64, t10470: f64, t558: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12275 = t12274 * t1499;
    let t12277 = t3728 * t3724;
    let t12279 = t12274 * t1491;
    let t12281 = t4161 * t1495;
    let t12286 = t1360 * t3960;
    let t12303 = t3245 * t1460;
    let t12305 = t10470 * t558;
    (t12275, t12277, t12279, t12281, t12286, t12303, t12305)
}
