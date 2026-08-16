//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 814/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk814(t1137: f64, t4819: f64, t1682: f64, t3359: f64, t1136: f64, t3238: f64, t3363: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64, t449: f64) -> (f64, f64, f64, f64, f64) {
    let t4820 = t4819 * t1137;
    let t4823 = t1682 * t3359;
    let t4824 = t4823 * t1136;
    let t4832 = t3363 - 0.30902777777777777778e-2_f64 * t3238 - 0.30902777777777777778e-2_f64 * t4721 - 0.61805555555555555555e-2_f64 * t4726 + 0.18541666666666666667e-1_f64 * t4731 + 0.92708333333333333333e-2_f64 * t4735;
    let t4833 = t4832 * t449;
    (t4820, t4823, t4824, t4832, t4833)
}
