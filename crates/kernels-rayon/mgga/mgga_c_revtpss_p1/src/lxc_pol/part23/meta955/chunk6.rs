//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3188/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3188(t1266: f64, t17290: f64, t21085: f64, t21137: f64, t21140: f64, t21213: f64, t5313: f64, t5327: f64, t5373: f64, t57727: f64, t6647: f64, t83719: f64, t83725: f64, t83728: f64, t83731: f64, t83735: f64) -> f64 {
    let t83741 = t5373 * t21137 / 9.0_f64 + t5373 * t21140 / 6.0_f64 + t83719 / 216.0_f64 + 11.0_f64 / 81.0_f64 * t21213 * t5313 + 0.35400176935018568008e-1_f64 * t83725 * t1266 + 0.22866142996303859718e-2_f64 * t83728 * t1266 - t57727 - 0.14481890564325777821e-1_f64 * t83731 - 0.14291339372689912324e-3_f64 * t83735 - 0.64311027177104605458e-3_f64 * t17290 * t6647 - 0.64311027177104605458e-3_f64 * t5327 * t21085;
    t83741
}
