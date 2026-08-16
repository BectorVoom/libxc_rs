//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1140/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1140(t33: f64, t1497: f64, t9868: f64, t2: f64, t3289: f64, t1006: f64, t555: f64, t22: f64, t2829: f64, t3226: f64, t4368: f64, t4371: f64, t493: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t12711 = t9868 * t1497;
    let t12714 = t3289 * t2;
    let t12715 = t555 * t1006;
    let t12725 = piecewise3(t34, 0.0_f64, -8.0_f64 / 27.0_f64 * t12711 * t3226 - 16.0_f64 / 9.0_f64 * t12714 * t12715 + 4.0_f64 / 9.0_f64 * t4368 * t2829 - 8.0_f64 / 3.0_f64 * t493 * t555 + 8.0_f64 * t4371 * t22);
    (t12715, t12725)
}
