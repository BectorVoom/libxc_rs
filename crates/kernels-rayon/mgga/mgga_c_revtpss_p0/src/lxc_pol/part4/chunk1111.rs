//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1111/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1111(t33: f64, t1711: f64, t9617: f64, t2: f64, t3881: f64, t1348: f64, t13569: f64, t22: f64, t3351: f64, t3842: f64, t5582: f64, t5585: f64, t580: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t13701 = t9617 * t1711;
    let t13704 = t3881 * t2;
    let t13714 = piecewise3(t34, 0.0_f64, 8.0_f64 / 27.0_f64 * t13701 * t3842 + 8.0_f64 / 9.0_f64 * t13704 * t13569 - 2.0_f64 / 9.0_f64 * t5582 * t3351 - 4.0_f64 / 3.0_f64 * t1348 * t580 + 4.0_f64 * t5585 * t22);
    t13714
}
