//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1304/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1304(t1081: f64, t7540: f64, t118953: f64, t25927: f64, t1649: f64, t6665: f64, t25374: f64, t89953: f64, t16596: f64, t25353: f64, t28: f64, t118454: f64, t23788: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t119737 = t1081 * t7540;
    let t119743 = t25927 * t118953;
    let t119746 = t1649 * t6665;
    let t119755 = t89953 * t25374;
    let t119763 = t25927 * t16596;
    let t119766 = t28 * t25353;
    let t119780 = t23788 * t118454;
    (t119737, t119743, t119746, t119755, t119763, t119766, t119780)
}
