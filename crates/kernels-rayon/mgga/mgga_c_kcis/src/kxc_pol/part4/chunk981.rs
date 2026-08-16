//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 981/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk981(t1014: f64, t3238: f64, t4585: f64, t85: f64, t349: f64, t1098: f64, t3290: f64, t3309: f64, t3255: f64, t3281: f64, t245: f64, t2840: f64, t347: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10257 = t1014 * t3238;
    let t10269 = t85 * t4585;
    let t10271 = 0.29201909629629629629e-3_f64 * t10269 * t349;
    let t10282 = t1098 * t3290;
    let t10284 = t1098 * t3309;
    let t10286 = t3255 * t3281;
    let t10292 = t2840 * t245 * t347;
    (t10257, t10269, t10271, t10282, t10284, t10286, t10292)
}
