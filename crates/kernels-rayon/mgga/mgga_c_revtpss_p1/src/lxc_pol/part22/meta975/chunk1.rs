//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3277/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3277(t2394: f64, t40862: f64, t40868: f64, t51110: f64, t51112: f64, t51121: f64, t51125: f64, t51135: f64, t5988: f64, t62236: f64, t62241: f64, t62246: f64, t62251: f64, t800: f64) -> f64 {
    let t62258 = -0.50820002809285328225e-4_f64 * t62236 - 0.2032800112371413129e-3_f64 * t51110 - 0.25410001404642664112e-4_f64 * t62241 - 0.16006300097412701803e-1_f64 * t51112 + 0.45351183609335988443e-1_f64 * t51121 + 0.22866142996303859718e-3_f64 * t62246 + 0.2032800112371413129e-3_f64 * t51125 + 0.22866142996303859718e-3_f64 * t51135 - 0.18071592998981862717e-4_f64 * t62251 + 5.0_f64 / 4.0_f64 * t40868 * t800 * t5988 * t2394 + 455.0_f64 / 324.0_f64 * t40862;
    t62258
}
