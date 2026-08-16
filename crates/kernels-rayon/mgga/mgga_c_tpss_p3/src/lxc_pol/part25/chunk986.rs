//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 986/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk986(t30: f64, t33: f64, t13334: f64, t13583: f64, t13588: f64, t1989: f64, t4360: f64, t490: f64, t580: f64, t5335: f64, t9868: f64, t3289: f64, t5059: f64, t1006: f64, t4368: f64, t493: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t13594 = piecewise3(t31, 0.0_f64, -8.0_f64 / 27.0_f64 * t13583 * t580 + 16.0_f64 / 9.0_f64 * t4360 * t1989 + 4.0_f64 / 9.0_f64 * t13588 * t580 + 4.0_f64 / 3.0_f64 * t490 * t13334);
    let t13595 = t9868 * t5335;
    let t13600 = t3289 * t5059;
    let t13603 = -t13334;
    let t13607 = piecewise3(t34, 0.0_f64, -8.0_f64 / 27.0_f64 * t13595 * t1006 - 16.0_f64 / 9.0_f64 * t4368 * t1989 + 4.0_f64 / 9.0_f64 * t13600 * t1006 + 4.0_f64 / 3.0_f64 * t493 * t13603);
    (t13594, t13603, t13607)
}
