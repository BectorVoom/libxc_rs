//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1827/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1827(t26426: f64, t81046: f64, t22690: f64, t7732: f64, t81195: f64, t22832: f64, t5234: f64, t1336: f64, t22759: f64, t5252: f64, t836: f64, t5293: f64, t80820: f64) -> (f64, f64, f64, f64, f64) {
    let t91078 = t81046 * t26426;
    let t91081 = t81195 * t22690 * t7732;
    let t91100 = t5234 * t22832;
    let t91113 = t1336 * t22759 * t836 * t5252;
    let t91120 = t80820 * t5293;
    (t91078, t91081, t91100, t91113, t91120)
}
