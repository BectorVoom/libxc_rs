//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2189/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2189(t30105: f64, t689: f64, t94395: f64, t94649: f64, t108133: f64, t108135: f64, t30071: f64, t7308: f64, t94378: f64, t94388: f64, t94392: f64, t97682: f64, t97687: f64, t97690: f64, t97698: f64, t97702: f64, t97707: f64) -> f64 {
    let t108138 = t30105 * t689;
    let t108139 = t94395 * t108138;
    let t108141 = t94649 * t108138;
    let t108145 = -t97682 + t97687 + t97690 - 0.4336814094102599731e0_f64 * t30071 * t7308 - t97698 - 0.72280234901709995518e-2_f64 * t108133 + 0.12851425765524037203e-1_f64 * t108135 - t97702 - t97707 - 0.96373646535613327357e-2_f64 * t94378 + 0.28912093960683998207e-1_f64 * t108139 - 0.51405703062096148813e-1_f64 * t108141 - 0.17135234354032049604e-2_f64 * t94388 + 0.22849835011101738147e-2_f64 * t94392;
    t108145
}
