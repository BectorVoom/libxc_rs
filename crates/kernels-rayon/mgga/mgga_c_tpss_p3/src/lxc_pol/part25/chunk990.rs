//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 990/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk990(t30: f64, t33: f64, t1197: f64, t13334: f64, t13646: f64, t13651: f64, t1989: f64, t4380: f64, t580: f64, t5335: f64, t9936: f64, t3225: f64, t5059: f64, t1006: f64, t1201: f64, t13603: f64, t4388: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t13657 = piecewise3(t31, 0.0_f64, 8.0_f64 / 27.0_f64 * t13646 * t580 - 8.0_f64 / 9.0_f64 * t4380 * t1989 - 2.0_f64 / 9.0_f64 * t13651 * t580 + 2.0_f64 / 3.0_f64 * t1197 * t13334);
    let t13658 = t9936 * t5335;
    let t13663 = t3225 * t5059;
    let t13669 = piecewise3(t34, 0.0_f64, 8.0_f64 / 27.0_f64 * t13658 * t1006 + 8.0_f64 / 9.0_f64 * t4388 * t1989 - 2.0_f64 / 9.0_f64 * t13663 * t1006 + 2.0_f64 / 3.0_f64 * t1201 * t13603);
    (t13657, t13669)
}
