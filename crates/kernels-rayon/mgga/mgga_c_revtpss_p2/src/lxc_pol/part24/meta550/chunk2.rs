//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1628/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1628(t14546: f64, t18677: f64, t39649: f64, t39652: f64, t51390: f64, t51403: f64, t51408: f64, t62684: f64, t62716: f64, t62723: f64, t76237: f64, t76242: f64, t76255: f64) -> f64 {
    let t87824 = 0.39029762157531132076e-1_f64 * t76237 + t39649 - t39652 - 0.23707617058567841754e2_f64 * t14546 * t18677 * t76242 - 0.7805952431506226415e-2_f64 * t62684 + 0.1040793657534163522e-1_f64 * t51390 - 0.11708928647259339623e0_f64 * t76255 - 0.68293547082294194357e-1_f64 * t51403 - 0.12142592671231907757e0_f64 * t51408 + 0.69394917116090352835e-2_f64 * t62716 - 0.69394917116090352835e-2_f64 * t62723;
    t87824
}
