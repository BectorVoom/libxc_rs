//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2198/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2198(t108307: f64, t7284: f64, t30100: f64, t689: f64, t25904: f64, t25899: f64, t25924: f64, t27837: f64, t27853: f64, t27858: f64, t27864: f64, t5774: f64, t7295: f64, t7920: f64, t94700: f64, t94703: f64, t94714: f64, t94726: f64, t94733: f64, t94823: f64, t97943: f64, t97945: f64, t97949: f64, t98340: f64) -> f64 {
    let t108332 = t7284 * t108307;
    let t108334 = t30100 * t689;
    let t108335 = t25904 * t108334;
    let t108337 = t25899 * t108334;
    let t108349 = t94700 - t94703 - 0.73171657588172351096e-2_f64 * t94714 + 0.52041769129231196772e1_f64 * t94823 * t98340 * t27864 + 0.72280234901709995518e-2_f64 * t108332 - 0.14456046980341999104e-1_f64 * t108335 + 0.25702851531048074406e-1_f64 * t108337 - 0.52041769129231196772e1_f64 * t7295 * t25924 * t7920 * t5774 - 0.11565819519348392139e-2_f64 * t94726 + 0.8673628188205199462e0_f64 * t27837 * t27853 + 0.8673628188205199462e0_f64 * t27837 * t27858 + t97943 + t97945 - 0.65049603595885220126e-3_f64 * t94733 - t97949;
    t108349
}
