//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 926/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk926(t32826: f64, t6562: f64, t794: f64, t22893: f64, t23164: f64, t32818: f64, t32827: f64, t6547: f64, t23168: f64, t32819: f64, t234: f64, t7510: f64) -> (f64, f64, f64, f64, f64) {
    let t118709 = t6562 * t794 * t32826;
    let t118727 = t23164 * t22893 * t32818;
    let t118738 = t6547 * t32827;
    let t118744 = t23168 * t32819;
    let t118747 = t234 * t7510;
    (t118709, t118727, t118738, t118744, t118747)
}
