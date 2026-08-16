//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 517/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk517(t1503: f64, t5694: f64, t31: f64, t4518: f64, t1466: f64, t4522: f64, t605: f64, t1182: f64, t221: f64, t1468: f64, t1184: f64, t5572: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5696 = 0.12805126321218922714e0_f64 * t5694 * t1503;
    let t5697 = t4518 * t31;
    let t5698 = t5697 * t1466;
    let t5699 = t605 * t4522;
    let t5700 = t5699 * t1182;
    let t5701 = t221 * t5700;
    let t5704 = t1468 * t1182;
    let t5705 = t221 * t5704;
    let t5709 = t221 * t5572 * t1184;
    (t5696, t5698, t5700, t5701, t5704, t5705, t5709)
}
