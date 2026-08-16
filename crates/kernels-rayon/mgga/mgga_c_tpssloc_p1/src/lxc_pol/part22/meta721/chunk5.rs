//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2349/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2349(t118: f64, t20800: f64, t2576: f64, t794: f64, t210: f64, t214: f64, t41209: f64, t41212: f64, t41217: f64, t59204: f64, t59206: f64, t59214: f64, t59216: f64, t59218: f64, t59221: f64, t59224: f64, t67282: f64, t787: f64) -> f64 {
    let t68131 = t2576 * t118 * t794 * t20800;
    let t68141 = -0.16666666666666666666e-2_f64 * t787 * t210 * t214 * t67282 + 0.8333333333333333333e-3_f64 * t68131 + t41209 + t41212 + 0.11666666666666666666e0_f64 * t59204 + 0.47499999999999999998e-1_f64 * t59206 + 0.24999999999999999999e-2_f64 * t59214 + 0.11666666666666666666e-1_f64 * t59216 - 0.15833333333333333333e-1_f64 * t59218 - 0.14999999999999999999e-1_f64 * t59221 + 0.49999999999999999998e-2_f64 * t59224 + 0.27777777777777777778e-3_f64 * t41217;
    t68141
}
