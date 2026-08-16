//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2528/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2528(t1164: f64, t4861: f64, t64525: f64, t21833: f64, t3411: f64, t18786: f64, t4874: f64, t21826: f64, t300: f64, t1166: f64, t22236: f64, t4883: f64) -> (f64, f64, f64, f64, f64) {
    let t71225 = 0.51947577317044391277e2_f64 * t1164 * t64525 * t4861;
    let t71227 = 0.35089341735807877242e1_f64 * t3411 * t21833;
    let t71230 = 0.35089341735807877242e1_f64 * t1164 * t4874 * t18786;
    let t71231 = t300 * t21826;
    let t71233 = 0.5848223622634646207e0_f64 * t71231 * t1166;
    let t71236 = 0.6233709278045326953e3_f64 * t1164 * t22236 * t4883;
    (t71225, t71227, t71230, t71233, t71236)
}
