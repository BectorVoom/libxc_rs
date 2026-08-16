//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1022/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1022(t12274: f64, t1491: f64, t1495: f64, t4161: f64, t1360: f64, t3960: f64, t1460: f64, t3245: f64, t10470: f64, t558: f64, t1014: f64, t4111: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12279 = t12274 * t1491;
    let t12281 = t4161 * t1495;
    let t12286 = t1360 * t3960;
    let t12303 = t3245 * t1460;
    let t12305 = t10470 * t558;
    let t12306 = 0.73697530864197530862e-3_f64 * t12305;
    let t12307 = t1014 * t4111;
    (t12279, t12281, t12286, t12303, t12305, t12306, t12307)
}
