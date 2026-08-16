//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2026/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2026(t84555: f64, t84558: f64, t91398: f64, t91400: f64, t91406: f64, t93762: f64, t93763: f64, t97435: f64, t97437: f64, t97439: f64, t97444: f64, t97447: f64, t97450: f64, t97453: f64, t97456: f64, t97459: f64, t97461: f64, t97463: f64) -> f64 {
    let t102746 = -0.96894614625936938048e-2_f64 * t97435 - t97437 / 24.0_f64 + 0.16956557559538964158e-1_f64 * t97439 - 35.0_f64 / 54.0_f64 * t91398 - 0.27130492095262342653e0_f64 * t91400 + t93762 + t93763 - t91406 + 0.28260929265898273597e-2_f64 * t97444 + 0.33913115119077928316e-1_f64 * t97447 + 0.16956557559538964158e-1_f64 * t97450 - t84555 + t84558 - t97453 / 2.0_f64 + t97456 / 4.0_f64 - 0.13565246047631171326e0_f64 * t97459 - t97461 / 128.0_f64 + 0.28260929265898273597e-2_f64 * t97463;
    t102746
}
