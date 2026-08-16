//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 951/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk951(t234: f64, t7823: f64, t23110: f64, t23185: f64, t33379: f64, t23168: f64, t33376: f64, t33380: f64, t6579: f64, t33384: f64, t6547: f64, t33429: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121506 = t234 * t7823;
    let t121524 = t23185 * t23110 * t33379;
    let t121533 = t23168 * t33376;
    let t121536 = t6579 * t33380;
    let t121574 = t6547 * t33384;
    let t121629 = t6547 * t33429;
    (t121506, t121524, t121533, t121536, t121574, t121629)
}
