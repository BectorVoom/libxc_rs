//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 959/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk959(t1184: f64, t1971: f64, t40427: f64, t515: f64, t7365: f64, t1182: f64, t618: f64, t236: f64, t7231: f64, t3352: f64, t38928: f64, t558: f64) -> (f64, f64, f64, f64, f64) {
    let t40431 = t7365 * t1971 * t515 * t40427 * t1184;
    let t40433 = t618 * t1182;
    let t40437 = t7365 * t7231 * t236 * t40433 * t1184;
    let t40442 = t7365 * t3352 * t236 * t38928 * t1184;
    let t40444 = t558 * t1182;
    (t40431, t40433, t40437, t40442, t40444)
}
