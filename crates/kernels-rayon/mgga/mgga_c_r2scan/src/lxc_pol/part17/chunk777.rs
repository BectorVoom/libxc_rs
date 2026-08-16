//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 777/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk777(t1610: f64, t2207: f64, t2691: f64, t2530: f64, t537: f64, t6217: f64, t7460: f64, t1632: f64, t2634: f64, t551: f64, t2184: f64, t2612: f64) -> (f64, f64, f64, f64, f64) {
    let t7500 = 0.34930954652346593434e-1_f64 * t2207 * t1610 * t2691;
    let t7503 = t537 * t2530;
    let t7512 = t6217 * t7460;
    let t7551 = t551 * t1632 * t2634;
    let t7553 = 0.46230515946956099004e0_f64 * t2184 * t7551;
    let t7555 = t551 * t1632 * t2612;
    (t7500, t7503, t7512, t7553, t7555)
}
