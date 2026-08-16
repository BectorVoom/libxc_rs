//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 818/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk818(t51: f64, t3007: f64, t4920: f64, t1224: f64, t3010: f64, t476: f64, t8584: f64, t1217: f64, t2517: f64, t419: f64, t8615: f64, zeta_threshold: f64) -> f64 {
    let t52 = t51 <= zeta_threshold;
    let t8616 = t4920 * t3007;
    let t8621 = t1224 * t3010;
    let t8624 = t476 * t8584;
    let t8627 = piecewise3(t52, 0.0_f64, 8.0_f64 / 27.0_f64 * t8616 * t419 + 8.0_f64 / 9.0_f64 * t2517 * t1217 - 2.0_f64 / 9.0_f64 * t8621 * t419 + 2.0_f64 / 3.0_f64 * t8624);
    let t8629 = t8615 / 2.0_f64 + t8627 / 2.0_f64;
    t8629
}
