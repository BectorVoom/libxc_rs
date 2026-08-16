//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3856/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3856(t47093: f64, t39989: f64, t47084: f64, t47086: f64, t47088: f64, t47092: f64, t47096: f64, t74114: f64, t74115: f64, t74116: f64, t74117: f64, t74119: f64, t74120: f64, t74121: f64, t74122: f64, t74123: f64, t74124: f64, t74125: f64) -> (f64, f64) {
    let t74126 = 0.20779030926817756511e3_f64 * t47093;
    let t74127 = -t74114 + t74115 + t74116 - t74117 + t74119 - t74120 - t47084 - t74121 + t74122 + t74123 + t74124 - t39989 - t47086 + t47088 - t74125 + t47092 + t74126 - t47096;
    (t74126, t74127)
}
