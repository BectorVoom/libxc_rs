//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1349/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1349(t65564: f64, t65567: f64, t67143: f64, t67150: f64, t69510: f64, t69512: f64, t69515: f64, t69517: f64, t69519: f64, t69521: f64, t69523: f64, t69525: f64, t69527: f64) -> f64 {
    let t71787 = t69510 / 96.0_f64 + t69512 / 96.0_f64 - t67143 - t65564 + t69515 / 192.0_f64 - 7.0_f64 / 144.0_f64 * t69517 + t69519 / 384.0_f64 + t69521 / 192.0_f64 - t69523 / 384.0_f64 - 35.0_f64 / 54.0_f64 * t65567 - 7.0_f64 / 24.0_f64 * t69525 + 7.0_f64 / 72.0_f64 * t69527 + t67150;
    t71787
}
