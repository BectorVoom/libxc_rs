//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 801/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk801(t51: f64, t4920: f64, t893: f64, t1224: f64, t35: f64, t1216: f64, t476: f64, t1225: f64, t1228: f64, t2517: f64, t2520: f64, t40: f64, t6995: f64, zeta_threshold: f64) -> f64 {
    let t52 = t51 <= zeta_threshold;
    let t7073 = t4920 * t893;
    let t7076 = t1224 * t35;
    let t7081 = t476 * t1216;
    let t7086 = piecewise3(t52, 0.0_f64, 8.0_f64 / 27.0_f64 * t7073 * t1225 + 8.0_f64 / 9.0_f64 * t7076 * t6995 - 2.0_f64 / 9.0_f64 * t2517 * t1228 - 4.0_f64 / 3.0_f64 * t7081 + 4.0_f64 * t2520 * t40);
    t7086
}
