//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3420/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3420(t41330: f64, t41332: f64, t63474: f64, t63476: f64, t63478: f64, t63480: f64, t63482: f64, t63485: f64, t63488: f64, t63491: f64, t63494: f64, t63497: f64, t63500: f64, t63503: f64, t63505: f64) -> f64 {
    let t64277 = -0.3529725e1_f64 * t63474 - 0.17648625e1_f64 * t63476 - 0.157790625e0_f64 * t63478 + 0.6311625e0_f64 * t63480 + 0.31558125e0_f64 * t63482 - 0.3529725e1_f64 * t63485 + 0.6311625e0_f64 * t63488 - 0.6618234375e1_f64 * t63491 + 0.264729375e1_f64 * t63494 + 0.2366859375e0_f64 * t63497 - 0.157790625e0_f64 * t63500 + 0.83356000000000000001e0_f64 * t63503 + 0.3529725e1_f64 * t63505 - 0.22954444444444444444e0_f64 * t41330 - 0.15302962962962962963e0_f64 * t41332;
    t64277
}
