//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1355/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1355(t13769: f64, t17794: f64, t17804: f64, t2986: f64, t340: f64, t343: f64, t4510: f64, t4531: f64, t61310: f64, t61313: f64, t69548: f64, t69647: f64, t69683: f64, t69686: f64, t69691: f64, t69699: f64, t69727: f64, t69739: f64, t69746: f64, t76593: f64, t76901: f64, t76922: f64, t973: f64, t974: f64) -> f64 {
    let t76943 = -0.16666666666666666666e-2_f64 * t2986 * t17804 * t17794 - 0.13333333333333333333e-1_f64 * t2986 * t4510 * t76593 + 0.88888888888888888886e-2_f64 * t2986 * t13769 * t69548 - 0.83333333333333333332e-3_f64 * t973 * t974 * t340 * (t76901 + t76922) * t343 - 0.22222222222222222221e-2_f64 * t69683 - 0.11111111111111111111e-2_f64 * t69686 - 0.11111111111111111111e-2_f64 * t69691 - 0.14814814814814814815e-2_f64 * t69699 - 0.29629629629629629628e-2_f64 * t69727 + 0.37037037037037037036e-3_f64 * t69739 + 0.66666666666666666664e-2_f64 * t2986 * t4531 * t69746 - 0.44444444444444444444e-2_f64 * t2986 * t13769 * t69647 + 0.11111111111111111111e-2_f64 * t61310 + 0.11111111111111111111e-2_f64 * t61313;
    t76943
}
